use debug::*;

use cpu::Cpu;

pub trait Mem {
    fn mem_get_short(&mut self, loc: u32) -> u8;
    fn mem_get_long(&mut self, loc: u32) -> u32;
    fn mem_set_short(&mut self, loc: u32, val: u8);
    fn mem_set_long(&mut self, loc: u32, val: u32);
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
            (self.mem[loc.wrapping_add(0) as usize] as u32) >> 0  |
            (self.mem[loc.wrapping_add(1) as usize] as u32) >> 8  |
            (self.mem[loc.wrapping_add(2) as usize] as u32) >> 16 |
            (self.mem[loc.wrapping_add(3) as usize] as u32) >> 24
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
            self.mem[loc.wrapping_add(0) as usize] = ((val >> 8) & 0xFF) as u8;
            self.mem[loc.wrapping_add(0) as usize] = ((val >> 16) & 0xFF) as u8;
            self.mem[loc.wrapping_add(0) as usize] = ((val >> 24) & 0xFF) as u8;
        }
    }
}
