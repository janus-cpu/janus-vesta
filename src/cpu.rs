use std::fs::File;
use std::io::Read;
use mem::Memory;
use instruction::Instruction;
use instruction::Operand;
use execute::Instructor;
use die::*;
// ------------------------------------------------------------------- //

pub struct Cpu {
    pub memory: Memory,
    pub registers: Registers,
    pub pc : u32,
    pub fault: Fault,
    pub interrupts: Vec<Interrupt>
}

impl Cpu {
    pub fn new(kernel_filename: String, memory_size: u32) -> Cpu {
        //Open the kernel file.
        let mut file = File::open(kernel_filename).die(INVALID_FILE);

        let mut v = Vec::new();
        file.read_to_end(&mut v).die(CANNOT_READ_FILE);

        let mut memory = Memory::new(memory_size);
        memory.load_kernel(v);

        Cpu { registers: Registers::new(),
              memory: memory,
              pc: 0,
              fault: Fault::FAULT_NONE,
              interrupts: Vec::new() }
    }

    pub fn execute(&mut self) {
        loop {
            let pc = self.pc;

            debug!("Instruction at: {}", pc);

            let inst_bytes = self.retrieve_mem_long(pc); //TODO: paging!

            if self.handle_fault_retrieve(pc) {
                continue;
            }

            let instruction = self.decode(inst_bytes);

            if self.handle_fault_decode(pc) {
                continue;
            }

            self.pc += 4; // Read 4 bytes.
            self.instruct(instruction);

            debug!("Registers after inst: {}", self.registers);

            let inst_size = instruction.instruction_size();

            if self.handle_fault_execute(pc) {
                continue;
            }

            self.handle_interrupt(pc + inst_size);
        }
    }

    pub fn decode(&mut self, mut inst_bytes: u32) -> Instruction {
        let opcode = inst_bytes & 0b111111111;
        inst_bytes >>= 9;
        let size = inst_bytes & 1;
        inst_bytes >>= 5;
        let type1 = inst_bytes & 0b11;
        inst_bytes >>= 2;
        let type2 = inst_bytes & 0b11;
        inst_bytes >>= 2;
        let op1 = inst_bytes & 0b1111111;
        inst_bytes >>= 7;
        let op2 = inst_bytes & 0b1111111;

        debug!("opcode: 0x{:X}, size: {}, type: {}, {}, op: {}, {}",
               opcode, size, type1, type2, op1, op2);

        let inst = Instruction::decode(opcode, size, type1, type2, op1, op2);

        if let Ok(i) = inst {
            debug!("{}", i);
        } else if let Err(_) = inst {
            debug!("Error decoding instruction...");
            // panic!("OH NO!");
        }

        return inst.unwrap_or_else(|_| {
                self.fault(Fault::FAULT_ILLEGAL_INSTRUCTION);
                Instruction::dead()
        });
    }
}

// ------------------------------------------------------------------- //

pub struct Registers {
    pub gp: [u32; 16],
    pub rr: u32,
    pub re: [u32; 7],
    pub rk: [u32; 8],

    pub rflags: u32,
    pub rm: u32,
    pub ri: u32
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            gp: [0; 16],
            rr: 0,
            re: [0; 7],
            rk: [0; 8],
            rflags: 0,
            rm: 0,
            ri: 0
        }
    }
}

impl Cpu {
    pub fn get_carry_flag(&self) -> bool {
        (self.registers.rflags >> 0) & 0b1 == 1
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.registers.rflags >> 1) & 0b1 == 1
    }

    pub fn get_neg_flag(&self) -> bool {
        (self.registers.rflags >> 2) & 0b1 == 1
    }

    pub fn get_ovf_flag(&self) -> bool {
        (self.registers.rflags >> 3) & 0b1 == 1
    }

    pub fn get_kernel_flag(&self) -> bool {
        (self.registers.rflags >> 4) & 0b1 == 1
    }

    pub fn get_handling_flag(&self) -> bool {
        (self.registers.rflags >> 5) & 0b1 == 1
    }

    pub fn get_mask_flag(&self) -> bool {
        (self.registers.rflags >> 6) & 0b1 == 1
    }
}

pub trait FlagOperations<T> {
    fn set_carry_flag(&mut self, a: T);
    fn set_zero_flag(&mut self, a: T);
    fn set_neg_flag(&mut self, a: T);
    fn set_ovf_flag(&mut self, a: T);
    fn set_kernel_flag(&mut self, a: T);
    fn set_handling_flag(&mut self, a: T);
    fn set_mask_flag(&mut self, a: T);
}

impl FlagOperations<u32> for Cpu {
    fn set_carry_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 0);
        k |= if a > 0 { 1 << 0 } else {0};
        self.registers.rflags = k;
    }

    fn set_zero_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 1);
        k |= if a > 0 { 1 << 1 } else {0};
        self.registers.rflags = k;
    }

    fn set_neg_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 2);
        k |= if a > 0 { 1 << 2 } else {0};
        self.registers.rflags = k;
    }

    fn set_ovf_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 3);
        k |= if a > 0 { 1 << 3 } else {0};
        self.registers.rflags = k;
    }

    fn set_kernel_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 4);
        k |= if a > 0 { 1 << 4 } else {0};
        self.registers.rflags = k;
    }

    fn set_handling_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 5);
        k |= if a > 0 { 1 << 5 } else {0};
        self.registers.rflags = k;
    }

    fn set_mask_flag(&mut self, a: u32) {
        let mut k = self.registers.rflags;
        k &= !(1 << 6);
        k |= if a > 0 { 1 << 6 } else {0};
        self.registers.rflags = k;
    }
}

impl FlagOperations<bool> for Cpu {
    fn set_carry_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 0);
        k |= if a { 1 << 0 } else {0};
        self.registers.rflags = k;
    }

    fn set_zero_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 1);
        k |= if a { 1 << 1 } else {0};
        self.registers.rflags = k;
    }

    fn set_neg_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 2);
        k |= if a { 1 << 2 } else {0};
        self.registers.rflags = k;
    }

    fn set_ovf_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 3);
        k |= if a { 1 << 3 } else {0};
        self.registers.rflags = k;
    }

    fn set_kernel_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 4);
        k |= if a { 1 << 4 } else {0};
        self.registers.rflags = k;
    }

    fn set_handling_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 5);
        k |= if a { 1 << 5 } else {0};
        self.registers.rflags = k;
    }

    fn set_mask_flag(&mut self, a: bool) {
        let mut k = self.registers.rflags;
        k &= !(1 << 6);
        k |= if a { 1 << 6 } else {0};
        self.registers.rflags = k;
    }
}

// ------------------------------------------------------------------- //

impl Cpu {
    pub fn retrieve_op_short(&mut self, op: Operand) -> u8 {
        match op {
            Operand::None => 0u8,
            Operand::Reg(r) => {
                self.retrieve_reg_short(r)
            },
            Operand::RegDeref(r) => {
                let regvalue = self.retrieve_reg_long(r);
                self.retrieve_mem_short(regvalue)
            },
            Operand::RegOff(r, o) => {
                let regvalue = self.retrieve_reg_long(r);
                let regoff = match o {
                    0 => regvalue,
                    1 => regvalue.wrapping_add(1),
                    2 => regvalue.wrapping_add(2),
                    3 => regvalue.wrapping_add(3),
                    4 => regvalue.wrapping_sub(3),
                    5 => regvalue.wrapping_sub(2),
                    6 => regvalue.wrapping_sub(1),
                    7 => {
                        let pc = self.pc;
                        let off_bits = self.retrieve_mem_long(pc);
                        self.pc += 4;
                        regvalue.wrapping_add(off_bits)
                    },
                    _ => { panic!("Illegal offset passed!"); }
                };

                self.retrieve_mem_short(regoff)
            },
            Operand::Const => {
                let pc = self.pc;
                let con = self.retrieve_mem_long(pc);
                self.pc += 4;
                con as u8
            }
        }
    }

    pub fn retrieve_op_long(&mut self, op: Operand) -> u32 {
        match op {
            Operand::None => 0u32,
            Operand::Reg(r) => {
                self.retrieve_reg_long(r)
            },
            Operand::RegDeref(r) => {
                let regvalue = self.retrieve_reg_long(r);
                self.retrieve_mem_long(regvalue)
            },
            Operand::RegOff(r, o) => {
                let regvalue = self.retrieve_reg_long(r);
                let regoff = match o {
                    0 => regvalue,
                    1 => regvalue.wrapping_add(4),
                    2 => regvalue.wrapping_add(8),
                    3 => regvalue.wrapping_add(12),
                    4 => regvalue.wrapping_sub(12),
                    5 => regvalue.wrapping_sub(8),
                    6 => regvalue.wrapping_sub(4),
                    7 => {
                        let pc = self.pc;
                        let off_bits = self.retrieve_mem_long(pc);
                        self.pc += 4;
                        regvalue.wrapping_add(off_bits)
                    },
                    _ => { unreachable!(); }
                };

                debug!("Regoff is {}", regoff);
                self.retrieve_mem_long(regoff)
            },
            Operand::Const => {
                let pc = self.pc;
                let con = self.retrieve_mem_long(pc);
                self.pc += 4;
                con
            }
        }
    }

    pub fn retrieve_reg_short(&mut self, reg: u32) -> u8 {
        let register = reg / 4;

        (self.registers.gp[register as usize] >> (8 * (reg % 4))) as u8
    }

    pub fn retrieve_reg_long(&mut self, reg: u32) -> u32 {
        match reg {
            0...15 => self.registers.gp[reg as usize],
            16    => self.registers.rr,
            17...23 => self.registers.re[(reg - 17) as usize],
            24...31 => self.registers.rk[(reg - 24) as usize],
            _ => { panic!("Illegal register passed!"); }
        }
    }

    pub fn retrieve_mem_short(&mut self, loc: u32) -> u8 {
        if let Ok(m) = self.memory.retrieve_short(loc) {
            m
        } else {
            self.fault(Fault::FAULT_INVALID_MEMORY_ACCESS(loc));
            0
        }
    }

    pub fn retrieve_mem_long(&mut self, loc: u32) -> u32 {
        if let Ok(m) = self.memory.retrieve_long(loc) {
            m
        } else {
            self.fault(Fault::FAULT_INVALID_MEMORY_ACCESS(loc));
            0
        }
    }

    pub fn store_op_short(&mut self, op: Operand, i: u8) {
        match op {
            Operand::None => { unreachable!(); },
            Operand::Reg(r) => {
                self.store_reg_short(r, i)
            },
            Operand::RegDeref(r) => {
                let regvalue = self.retrieve_reg_long(r);
                self.store_mem_short(regvalue, i)
            },
            Operand::RegOff(r, o) => {
                let regvalue = self.retrieve_reg_long(r);
                let regoff = match o {
                    0 => regvalue,
                    1 => regvalue.wrapping_add(1),
                    2 => regvalue.wrapping_add(2),
                    3 => regvalue.wrapping_add(3),
                    4 => regvalue.wrapping_sub(3),
                    5 => regvalue.wrapping_sub(2),
                    6 => regvalue.wrapping_sub(1),
                    7 => {
                        let pc = self.pc;
                        let off_bits = self.retrieve_mem_long(pc);
                        self.pc += 4;
                        regvalue.wrapping_add(off_bits)
                    },
                    _ => { panic!("Illegal offset passed!"); }
                };

                self.store_mem_short(regoff, i)
            },
            Operand::Const => { panic!("Cannot store into const!"); }
        }
    }

    pub fn store_op_long(&mut self, op: Operand, i: u32) {
        match op {
            Operand::None => { panic!("Cannot store into none") },
            Operand::Reg(r) => {
                self.store_reg_long(r, i)
            },
            Operand::RegDeref(r) => {
                let regvalue = self.retrieve_reg_long(r);
                self.store_mem_long(regvalue, i)
            },
            Operand::RegOff(r, o) => {
                let regvalue = self.retrieve_reg_long(r);
                let regoff = match o {
                    0 => regvalue,
                    1 => regvalue.wrapping_add(4),
                    2 => regvalue.wrapping_add(8),
                    3 => regvalue.wrapping_add(12),
                    4 => regvalue.wrapping_sub(12),
                    5 => regvalue.wrapping_sub(8),
                    6 => regvalue.wrapping_sub(4),
                    7 => {
                        let pc = self.pc;
                        let off_bits = self.retrieve_mem_long(pc);
                        self.pc += 4;
                        regvalue.wrapping_add(off_bits)
                    },
                    _ => { panic!("Illegal offset passed!"); }
                };

                self.store_mem_long(regoff, i)
            },
            Operand::Const => { panic!("Cannot store into const!"); }
        }
    }

    pub fn store_reg_short(&mut self, reg: u32, i: u8) {
        let register = reg / 4;
        let byte = reg % 4;

        let mut k = self.registers.gp[register as usize];
        k &= !(0xFF << (8 * byte));
        k |= (i as u32) << (8 * byte);
        self.registers.gp[register as usize] = k;
    }

    pub fn store_reg_long(&mut self, reg: u32, i: u32) {
        match reg {
            0...15 => self.registers.gp[reg as usize] = i,
            16    => self.registers.rr = i,
            17...23 => self.registers.re[(reg - 17) as usize] = i,
            24...31 => self.registers.rk[(reg - 24) as usize] = i, //TODO: privilege fault for rk0..rk7
            _ => { panic!("Illegal register passed!"); }
        }
    }

    //TODO: change to store_mem_short_{raw, mmu} and fix case-by-case w compiler errs
    pub fn store_mem_short(&mut self, loc: u32, i: u8) {
        if let Err(_) = self.memory.store_short(loc, i) {
            self.fault(Fault::FAULT_INVALID_MEMORY_ACCESS(loc));
        }
    }

    pub fn store_mem_long(&mut self, loc: u32, i: u32) {
        if let Err(_) = self.memory.store_long(loc, i) {
            self.fault(Fault::FAULT_INVALID_MEMORY_ACCESS(loc));
        }
    }
}

// ------------------------------------------------------------------- //

impl Cpu {
    pub fn fault(&mut self, fault: Fault) {
        self.check_double_fault();
        self.fault = fault;
        //TODO: skip maskable faults
    }

    /* The only fault that can occur when retrieving an instruction byte
     * is an INVALID_MEMORY_ACCESS fault. */
    pub fn handle_fault_retrieve(&mut self, pc: u32) -> bool {
        match self.fault {
            Fault::FAULT_INVALID_MEMORY_ACCESS(location) => {
                debug!("Invalid memory access at {}", location);
                self.fault = Fault::FAULT_NONE;
                let rflags = self.registers.rflags;

                if self.get_kernel_flag() {
                    // Swap rs with rk0
                    let rk0 = self.registers.rk[0];
                    self.registers.rk[0] = self.registers.gp[15];
                    self.registers.gp[15] = rk0;
                    // Enable privileged mode and the handling flag
                    self.set_kernel_flag(true);
                }

                let mut rs = self.registers.gp[15];
                self.set_handling_flag(true);
                self.set_mask_flag(true);

                // Push return pc and fault location
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, rflags);
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, 0);
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, pc);
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, location);
                self.registers.gp[15] = rs;

                let ri = self.registers.ri;
                let offset = self.retrieve_mem_long(ri + 4); /* INT 0x1 */
                self.pc = offset;

                // And tell the CPU to stop executing the current instruction
                return true;
            },
            Fault::FAULT_NONE => { return false; }
            _ => { unreachable!(); }
        }
    }

    /* The only fault that can occur when decoding an instruction byte is
       an ILLEGAL_INSTRUCTION fault. */
    pub fn handle_fault_decode(&mut self, pc: u32) -> bool {
        match self.fault {
            Fault::FAULT_ILLEGAL_INSTRUCTION => {
                self.fault = Fault::FAULT_NONE;
                let rflags = self.registers.rflags;

                if self.get_kernel_flag() {
                    // Swap rs with rk0
                    let rk0 = self.registers.rk[0];
                    self.registers.rk[0] = self.registers.gp[15];
                    self.registers.gp[15] = rk0;
                    // Enable privileged mode and the handling flag
                    self.set_kernel_flag(true);
                }

                let mut rs = self.registers.gp[15];
                self.set_handling_flag(true);
                self.set_mask_flag(true);

                // Push old rflags and return pc
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, rflags);
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, pc);
                self.registers.gp[15] = rs;

                let ri = self.registers.ri;
                let offset = self.retrieve_mem_long(ri + 0); /* INT 0x0 */
                self.pc = offset;

                // And tell the CPU to stop executing the current instruction
                return true;
            },
            Fault::FAULT_NONE => { return false; }
            _ => { unreachable!(); }
        }
    }

    /* Handles faults which will happen after executing an instruction */
    pub fn handle_fault_execute(&mut self, pc: u32) -> bool {
        match self.fault {
            Fault::FAULT_INVALID_MEMORY_ACCESS(location) => {
                self.fault = Fault::FAULT_NONE;
                let rflags = self.registers.rflags;

                if self.get_kernel_flag() {
                    // Swap rs with rk0
                    let rk0 = self.registers.rk[0];
                    self.registers.rk[0] = self.registers.gp[15];
                    self.registers.gp[15] = rk0;
                    // Enable privileged mode and the handling flag
                    self.set_kernel_flag(true);
                }

                let mut rs = self.registers.gp[15];
                self.set_handling_flag(true);
                self.set_mask_flag(true);

                // Push return pc and fault location
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, rflags);
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, pc);
                rs = rs.wrapping_sub(4);
                self.store_mem_long(rs, location);
                self.registers.gp[15] = rs;

                let ri = self.registers.ri;
                let offset = self.retrieve_mem_long(ri + 4); /* INT 0x1 */
                self.pc = offset;

                // And tell the CPU to stop executing the current instruction
                return true;
            },
            Fault::FAULT_HALT => {
                die("Halted.");
            }
            Fault::FAULT_NONE => { return false; }
            _ => { unreachable!(); }
        }
    }

    fn handle_interrupt(&mut self, pc: u32) {
        debug!("Trying to handle {} interrupts", self.interrupts.len());

        if self.get_mask_flag() {
            return;
        }

        if self.interrupts.len() > 0 {
            let int = self.interrupts.remove(0);
            let rflags = self.registers.rflags;

            if !self.get_kernel_flag() {
                //TODO: this may be broken
                // Swap rs with rk0
                let rk0 = self.registers.rk[0];
                self.registers.rk[0] = self.registers.gp[15];
                self.registers.gp[15] = rk0;
                // Enable privileged mode and the handling flag
                self.set_kernel_flag(true);
            }

            let mut rs = self.registers.gp[15];
            self.set_mask_flag(true);

            // Push return pc and fault location
            rs = rs.wrapping_sub(4);
            self.store_mem_long(rs, rflags);
            rs = rs.wrapping_sub(4);
            self.store_mem_long(rs, pc);

            let interrupt_offset;

            match int {
                Interrupt::INT_KEYPRESS(key) => {
                    rs = rs.wrapping_sub(1);
                    self.store_mem_short(rs, key);
                    interrupt_offset = 0x2;
                },
                Interrupt::INT_PHONY => {
                    interrupt_offset = 0x3;
                }
            }

            self.registers.gp[15] = rs;

            let ri = self.registers.ri;
            let offset = self.retrieve_mem_long(ri + interrupt_offset * 4); /* INT 0x1 */
            self.pc = offset;
        }
    }

    fn check_double_fault(&mut self) {
        if self.get_handling_flag() {
            die("Double fault in fault handler!");
        }
    }

    pub fn interrupt(&mut self, int_id: u32) {
        let r0 = self.registers.gp[0];

        match int_id {
            0 => { self.fault(Fault::FAULT_ILLEGAL_INSTRUCTION); }
            1 => { self.fault(Fault::FAULT_INVALID_MEMORY_ACCESS(r0)); }
            2 => { self.interrupts.push(Interrupt::INT_KEYPRESS((r0 & 0xFF) as u8)); }
            3 => { self.interrupts.push(Interrupt::INT_PHONY); }
            _ => { self.fault(Fault::FAULT_BAD_INTERRUPT); }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone)]
pub enum Fault {
    FAULT_NONE,
    /* 0x0 */ FAULT_ILLEGAL_INSTRUCTION,
    /* 0x1 */ FAULT_INVALID_MEMORY_ACCESS(u32),
    /* 0x4 */ FAULT_BAD_INTERRUPT,
    FAULT_HALT
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone)]
pub enum Interrupt {
    /* 0x2 */ INT_KEYPRESS(u8), /* INT 0x2 */
    /* 0x3 */ INT_PHONY
}
