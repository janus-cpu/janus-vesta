use cpu::Cpu;
use mem::Mem;
use flag::*;

pub const MEMORY_INTERRUPT: u8 = 0;
pub const PROTECT_INTERRUPT: u8 = 1;
pub const INSTRUCTION_INTERRUPT: u8 = 2;
pub const HALT_INTERRUPT: u8 = 3;

pub trait Interrupt {
    fn has_memory_interrupt(&self) -> bool;
    fn has_protect_interrupt(&self) -> bool;
    fn has_instruction_interrupt(&self) -> bool;
    fn has_scheduled_interrupt(&self) -> bool;

    fn trigger_memory_interrupt(&mut self);
    fn trigger_protect_interrupt(&mut self);
    fn trigger_instruction_interrupt(&mut self);
    fn trigger_next_interrupt(&mut self);

    fn trigger_interrupt(&mut self, int: u8);
}

impl Interrupt for Cpu {
    fn has_memory_interrupt(&self) -> bool {
        self.mem_interrupt_address.is_some()
    }

    fn has_protect_interrupt(&self) -> bool {
        self.protect_interrupt
    }

    fn has_instruction_interrupt(&self) -> bool {
        self.instr_interrupt
    }

    fn has_scheduled_interrupt(&self) -> bool {
        !self.interrupt_queue.is_empty()
    }

    fn trigger_memory_interrupt(&mut self) {
        let mem_addr = self.mem_interrupt_address.unwrap();
        self.mem_interrupt_address = None;

        self.rm = mem_addr;
        self.trigger_interrupt(MEMORY_INTERRUPT);
    }

    fn trigger_protect_interrupt(&mut self) {
        self.protect_interrupt = false;
        self.trigger_interrupt(PROTECT_INTERRUPT);
    }

    fn trigger_instruction_interrupt(&mut self) {
        self.instr_interrupt = false;
        self.trigger_interrupt(INSTRUCTION_INTERRUPT);
    }

    fn trigger_next_interrupt(&mut self) {
        let interrupt = self.interrupt_queue.pop_front().unwrap();
        self.trigger_interrupt(interrupt);
    }

    fn trigger_interrupt(&mut self, int: u8) {
        let rflags = self.rflags;
        let rp = self.rp;

        if self.flag_get(PROTECT_FLAG) {
            self.flag_set(PROTECT_FLAG, false);
            let rs = self.reg[15];
            self.reg[15] = self.rks;
            self.push_stack(rs);
        }

        self.push_stack(rflags);
        self.push_stack(rp);

        self.flag_set(EXTERNAL_FLAG, false);

        let off = self.ri + int as u32 * 4;
        let jmp = self.mem_get_long(off);

        if self.has_memory_interrupt() {
            fatal!("Double fault while handling 0x{:X}.", int);
        }

        self.rp = jmp;
    }
}
