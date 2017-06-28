use wrapping_util::WrappingIncrement;
use cpu::Cpu;

pub trait Mem {
    fn mem_get_short(&mut self, loc: u32) -> u8;
    fn mem_get_long(&mut self, loc: u32) -> u32;
    fn mem_set_short(&mut self, loc: u32, val: u8);
    fn mem_set_long(&mut self, loc: u32, val: u32);

    fn push_stack(&mut self, word: u32);
    fn pop_stack(&mut self) -> u32;
}

impl Mem for Cpu {
    fn mem_get_short(&mut self, loc: u32) -> u8 {
        if loc as usize > self.mem.len() {
            if !self.mem_interrupt_address.is_some() {
                self.mem_interrupt_address = Some(loc);
            }

            debug!("Memory access out of bounds @ 0x{:X}", loc);
            0
        } else {
            debug!("Reading mem short at {}", loc);
            self.mem[loc as usize]
        }
    }

    fn mem_get_long(&mut self, loc: u32) -> u32 {
        if loc.wrapping_add(4) < loc ||
           loc.wrapping_add(4) as usize > self.mem.len() {
            if !self.mem_interrupt_address.is_some() {
                self.mem_interrupt_address = Some(loc);
            }

            debug!("Memory access out of bounds @ 0x{:X} (long)", loc);
            0
        } else {
            debug!("Reading mem long at {}", loc);
            (self.mem[loc.wrapping_add(0) as usize] as u32) << 0  |
            (self.mem[loc.wrapping_add(1) as usize] as u32) << 8  |
            (self.mem[loc.wrapping_add(2) as usize] as u32) << 16 |
            (self.mem[loc.wrapping_add(3) as usize] as u32) << 24
        }
    }

    fn mem_set_short(&mut self, loc: u32, val: u8) {
        if loc as usize > self.mem.len() {
            if !self.mem_interrupt_address.is_some() {
                self.mem_interrupt_address = Some(loc);
            }

            debug!("Memory access out of bounds @ 0x{:X}", loc);
        } else {
            self.mem[loc as usize] = val;
        }
    }

    fn mem_set_long(&mut self, loc: u32, val: u32) {
        if loc.wrapping_add(4) < loc ||
           loc.wrapping_add(4) as usize > self.mem.len() {
            if !self.mem_interrupt_address.is_some() {
                self.mem_interrupt_address = Some(loc);
            }

            debug!("Memory access out of bounds @ 0x{:X} (long)", loc);
        } else {
            self.mem[loc.wrapping_add(0) as usize] = ((val >> 0) & 0xFF) as u8;
            self.mem[loc.wrapping_add(1) as usize] = ((val >> 8) & 0xFF) as u8;
            self.mem[loc.wrapping_add(2) as usize] = ((val >> 16) & 0xFF) as u8;
            self.mem[loc.wrapping_add(3) as usize] = ((val >> 24) & 0xFF) as u8;
        }
    }

    fn push_stack(&mut self, word: u32) {
        self.reg[15].wrapping_decrement(4);
        let rs = self.reg[15];
        self.mem_set_long(rs, word);
        debug!("Pushing {} to {}", word, rs);
    }

    fn pop_stack(&mut self) -> u32 {
        let rs = self.reg[15];
        let stk = self.mem_get_long(rs);
        self.reg[15].wrapping_increment(4);

        debug!("Popping {} from {}", stk, rs);
        stk
    }
}
