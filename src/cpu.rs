use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use wrapping_util::WrappingIncrement;

use debug::*;

use operation::{Operation, OperandParse};
use interrupt::Interrupt;
use mem::Mem;
use execute::Execute;
use flag::{Flag, EXTERNAL_FLAG};

pub struct Cpu {
    /// General-purpose registers r0-r15.
    pub reg: [u32; 16],

    /// Flags register. See `Flag` trait for more details.
    pub rflags: u32,
    /// Memory descriptor table register.
    pub rm: u32,
    /// Interrupt descriptor table register.
    pub ri: u32,
    /// Program counter register.
    pub rp: u32,
    /// Kernel stack register.
    pub rks: u32,
    /// Kernel thread register.
    pub rkt: u32,
    /// Fault address register.
    pub rf: u32,

    /// CPU's memory.
    pub mem: Vec<u8>,

    /// If a MEMORY interrupt occurred, this will hold the value
    /// of the address for which the interrupt was raised.
    pub mem_interrupt_address: Option<u32>,
    /// Has an INSTRUCTION interrupt occurred?
    pub instr_interrupt: bool,
    /// Has a PROTECT interrupt occurred?
    pub protect_interrupt: bool,
    /// Queue holding other scheduled general interrupts.
    pub interrupt_queue: VecDeque<u8>
}

impl Cpu {
    pub fn new(kernel_file: &str, mem_size: u32) -> Cpu {
        if mem_size < 128 {
            fatal!("There should be more than 128 bytes of memory!");
        }

        let mut cpu = Cpu {
            reg: [0; 16],
            rflags: 0,
            rm: 0,
            ri: 0,
            rp: 0,
            rks: 0,
            rkt: 0,
            rf: 0,
            mem: vec![0; mem_size as usize],
            mem_interrupt_address: None,
            instr_interrupt: false,
            protect_interrupt: false,
            interrupt_queue: VecDeque::new()
        };

        let mut file = File::open(kernel_file).unwrap_or_die(INVALID_FILE);

        // Load the file into a temporary vector
        let mut v = Vec::new();
        file.read_to_end(&mut v).unwrap_or_die(CANNOT_READ_FILE);

        if v.len() > mem_size as usize {
            fatal!("Kernel file too large to fit in memory!");
        }

        // Then bitwise-copy the vector into the CPU's memory.
        for (i, b) in v.into_iter().enumerate() {
            debug!("Mem init: Copying byte 0x{:X} to {}", b, i);
            cpu.mem[i] = b;
        }

        cpu
    }

    pub fn boot(mut self) -> ! {
        loop {
            // Save the old rp, if we interrupt.
            let rp = self.rp;
            let opcode = self.mem_get_short(rp);

            // Handle MEMORY interrupt retrieving opcode.
            if self.has_memory_interrupt() {
                debug!("Memory interrupt while reading opcode.");
                self.trigger_memory_interrupt();
                continue;
            }

            let operation;

            // Decode opcode, or fault with INSTRUCTION interrupt.
            if let Some(o) = Operation::decode(opcode) {
                operation = o;
            } else {
                self.trigger_instruction_interrupt();
                continue;
            }

            debug!("Decoded {} operation", operation);

            // Increment past opcode, then decode operands.
            self.rp.wrapping_increment(1);
            let (op1, op2) = self.decode_operands(operation);

            // If we faulted with either INSTRUCTION or MEM, handle those.
            if self.has_memory_interrupt() {
                debug!("Memory interrupt while decoding operands.");
                self.rp = rp;
                self.trigger_memory_interrupt();
                continue;
            } else if self.has_instruction_interrupt() {
                debug!("Instruction interrupt while decoding operands.");
                self.rp = rp;
                self.trigger_instruction_interrupt();
                continue;
            }

            self.execute_operation(operation, op1, op2);

            // Handle MEMORY, INSTRUCTION, and PROTECT interrupts first
            // then schedule a fault if there is one and the EXTERNAL flag is set.
            if self.has_memory_interrupt() {
                self.rp = rp;
                self.trigger_memory_interrupt();
            } else if self.has_instruction_interrupt() {
                self.rp = rp;
                self.trigger_instruction_interrupt();
            } else if self.has_protect_interrupt() {
                self.rp = rp;
                self.trigger_protect_interrupt();
            } else if self.flag_get(EXTERNAL_FLAG)
                    && self.has_scheduled_interrupt() {
                debug!("Scheduling fault from queue.");
                self.trigger_next_interrupt();
            }
        }
    }
}
