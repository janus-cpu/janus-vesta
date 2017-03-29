use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use wrapping_util::WrappingIncrement;

use debug::*;

use operation::{Operation, OperandParse};
use interrupt::Interrupt;
use mem::Mem;
use execute::Execute;
use flag::*;

pub struct Cpu {
    pub reg: [u32; 16],

    pub rflags: u32,
    pub rm: u32,
    pub ri: u32,
    pub rp: u32,
    pub rks: u32,
    pub rkt: u32,
    pub rf: u32,

    pub mem: Vec<u8>,

    pub mem_interrupt_address: Option<u32>,
    pub instr_interrupt: bool,
    pub protect_interrupt: bool,
    pub interrupt_queue: VecDeque<u8>
}

impl Cpu {
    pub fn new(kernel_file: &str, mem_size: u32) -> Cpu {
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

        let mut v = Vec::new();
        file.read_to_end(&mut v).unwrap_or_die(CANNOT_READ_FILE);

        if v.len() > mem_size as usize {
            fatal!("Kernel file too large to fit in memory!");
        }

        for i in 0..v.len() {
            debug!("Copying byte 0x{:X} to index {}", v[i], i);
            cpu.mem[i] = v[i];
        }

        cpu
    }

    pub fn boot(mut self) -> ! {
        loop {
            // Save the old rp, if we interrupt.
            let rp = self.rp;
            let opcode = self.mem_get_long(rp);

            if self.has_memory_interrupt() {
                debug!("Memory interrupt while reading opcode.");
                self.trigger_memory_interrupt();
                continue;
            }

            let operation;

            if let Some(o) = Operation::decode(opcode) {
                operation = o;
            } else {
                self.trigger_instruction_interrupt();
                continue;
            }

            self.rp.wrapping_increment(4);
            let (op1, op2) = self.decode_operands(operation);

            if self.has_memory_interrupt() {
                debug!("Memory interrupt while decoding operands.");
                self.rp = rp;
                self.trigger_memory_interrupt();
                continue;
            } else if self.has_instruction_interrupt() {
                debug!("Instruction interrupt while decoding operands: {} {} {}.", operation, op1, op2);
                self.rp = rp;
                self.trigger_instruction_interrupt();
                continue;
            }

            self.execute_operation(operation, op1, op2);

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

    pub fn push_stack(&mut self, word: u32) {
        self.reg[15].wrapping_decrement(4);
        let rs = self.reg[15];
        self.mem_set_long(rs, word);
    }
}
