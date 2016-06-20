use die::*;

pub struct Memory {
    memory: Vec<u8>,
    size: u32
}

impl Memory {
    pub fn new(size: u32) -> Memory {
        Memory { memory: vec![0; size as usize], size: size }
    }

    pub fn load_kernel(&mut self, kernel: Vec<u8>) {
        if kernel.len() > (self.size as usize) {
            die(INSUFFICIENT_MEMORY);
        }

        for i in 0..kernel.len() {
            self.memory[i] = kernel[i];
        }
    }

    pub fn retrieve_short(&self, location: u32) -> Result<u8, ()> {
        if location < self.size {
            Ok(self.memory[location as usize])
        } else {
            Err(())
        }
    }

    pub fn retrieve_long(&self, location: u32) -> Result<u32, ()> {
        if location + 3 < self.size {
            let a = self.memory[(location + 0) as usize] as u32;
            let b = self.memory[(location + 1) as usize] as u32;
            let c = self.memory[(location + 2) as usize] as u32;
            let d = self.memory[(location + 3) as usize] as u32;

            Ok((a) | (b << 8) | (c << 16) | (d << 24))
        } else {
            panic!("Invalid memory location {}", location);
            Err(())
        }
    }

    pub fn store_short(&mut self, location: u32, byte: u8) -> Result<(), ()> {
        if location < self.size {
            self.memory[location as usize] = byte;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn store_long(&mut self, location: u32, integer: u32) -> Result<(), ()> {
        if location + 3 < self.size {
            let a = (integer >>  0) as u8;
            let b = (integer >>  8) as u8;
            let c = (integer >> 16) as u8;
            let d = (integer >> 24) as u8;

            self.memory[(location + 0) as usize] = a;
            self.memory[(location + 1) as usize] = b;
            self.memory[(location + 2) as usize] = c;
            self.memory[(location + 3) as usize] = d;

            Ok(())
        } else {
            Err(())
        }
    }
}
