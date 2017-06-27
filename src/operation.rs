use std::fmt;

use debug::*;
use wrapping_util::WrappingIncrement;
use cpu::Cpu;
use mem::Mem;
use interrupt::Interrupt;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    ADD, ADDS,
    SUB, SUBS,
    ADC, ADCS,
    SBB, SBBS,
    RSUB, RSUBS,
    NOR, NORS,
    NAND, NANDS,
    OR, ORS,
    ORN, ORNS,
    AND, ANDS,
    ANDN, ANDNS,
    XNOR, XNORS,
    NOT, NOTS,
    XOR, XORS,
    CMP, CMPS,
    TEST, TESTS,
    JMP,
    JE, JNE, JL, JLE, JG, JGE, JLU, JLEU, JGU, JGEU,
    CALL,
    RET,
    HLT,
    INT,
    IRET,
    LOM, ROM,
    LOI, ROI,
    ROP,
    LFL, RFL,
    LOT, ROT,
    LOS, ROS,
    LOF, ROF,

    MOV,
    MOVS,

    POP, POPS,
    PUSH, PUSHS,
    PUSHR, POPR,
    IN, INS,
    OUT, OUTS,
    XCHG, XCHGS,
    MOVE, MOVNE, MOVL, MOVLE, MOVG, MOVGE, MOVLU, MOVLEU, MOVGU, MOVGEU,
    MOVES, MOVNES, MOVLS, MOVLES, MOVGS, MOVGES, MOVLUS, MOVLEUS, MOVGUS, MOVGEUS
}

#[derive(Debug, Copy, Clone)]
pub enum Prototype {
    N, A, X, I, P, U, T
}

impl Operation {
    pub fn decode(opcode: u8) -> Option<Operation> {
        use self::Operation::*;

        Some(match opcode {
            0x00 => ADD,
            0x01 => ADDS,
            0x02 => SUB,
            0x03 => SUBS,
            0x04 => ADC,
            0x05 => ADCS,
            0x06 => SBB,
            0x07 => SBBS,
            0x08 => RSUB,
            0x09 => RSUBS,
            0x20 => NOR,
            0x21 => NORS,

            0x24 => NAND,
            0x25 => NANDS,

            0x28 => OR,
            0x29 => ORS,
            0x2A => ORN,
            0x2B => ORNS,
            0x2C => AND,
            0x2D => ANDS,
            0x2E => ANDN,
            0x2F => ANDNS,

            0x34 => XNOR,
            0x35 => XNORS,

            0x38 => NOT,
            0x39 => NOTS,

            0x3C => XOR,
            0x3D => XORS,

            0x42 => CMP,
            0x43 => CMPS,

            0x6C => TEST,
            0x6D => TESTS,

            0x80 => JMP,
            0x81 => JE,
            0x82 => JNE,
            0x83 => JL,
            0x84 => JLE,
            0x85 => JG,
            0x86 => JGE,
            0x87 => JLU,
            0x88 => JLEU,
            0x89 => JGU,
            0x8A => JGEU,
            0x8B => CALL,
            0x8C => RET,
            0x8D => HLT,
            0x8E => INT,
            0x8F => IRET,

            0x70 => LOM,
            0x71 => ROM,
            0x72 => LOI,
            0x73 => ROI,
            0x75 => ROP,
            0x76 => LFL,
            0x77 => RFL,
            0x78 => LOT,
            0x79 => ROT,
            0x7A => LOS,
            0x7B => ROS,
            0x7C => LOF,
            0x7D => ROF,

            0x30 => MOV,
            0x31 => MOVS,

            0xA0 => POP,
            0xA1 => POPS,
            0xA2 => PUSH,
            0xA3 => PUSHS,
            0xA4 => IN,
            0xA5 => INS,
            0xA6 => OUT,
            0xA7 => OUTS,
            0xA8 => XCHG,
            0xA9 => XCHGS,
            0xAA => POPR,
            0xAB => PUSHR,

            0xB0 => MOVE,
            0xB1 => MOVES,
            0xB2 => MOVNE,
            0xB3 => MOVNES,
            0xB4 => MOVL,
            0xB5 => MOVLS,
            0xB6 => MOVLE,
            0xB7 => MOVLES,
            0xB8 => MOVG,
            0xB9 => MOVGS,
            0xBA => MOVGE,
            0xBB => MOVGES,
            0xBC => MOVLU,
            0xBD => MOVLUS,
            0xBE => MOVLEU,
            0xBF => MOVLEUS,
            0xC0 => MOVGU,
            0xC1 => MOVGUS,
            0xC2 => MOVGEU,
            0xC3 => MOVGEUS,

            _ => return None
        })
    }

    pub fn prototype(operation: Operation) -> Prototype {
        use self::Operation::*;
        use self::Prototype::*;

        match operation {
            ADD | ADDS |
            SUB | SUBS |
            ADC | ADCS |
            SBB | SBBS |
            RSUB | RSUBS |
            NOR | NORS |
            NAND | NANDS |
            OR | ORS |
            ORN | ORNS |
            AND | ANDS |
            ANDN | ANDNS |
            XNOR | XNORS |
            XOR | XORS |
            CMP | CMPS |
            TEST | TESTS => A,

            NOT | NOTS => P,

            JMP | JE | JNE | JL |
            JLE | JG | JGE | JLU |
            JLEU | JGU | JGEU | CALL => U,

            RET | HLT | IRET => N,
            INT => T,

            LOM | LOI | LFL | LOT | LOS | LOF => U,
            ROM | ROI | ROP | RFL | ROT | ROS | ROF => P,

            MOV | MOVS | MOVE | MOVNE |
            MOVLE | MOVGE | MOVLU | MOVLEU |
            MOVGU | MOVGEU | MOVES | MOVNES |
            MOVLES | MOVGES | MOVLUS | MOVLEUS |
            MOVGUS | MOVGEUS | MOVL | MOVLS |
            MOVG | MOVGS => A,

            POP | POPS => P,
            PUSH | PUSHS => U,
            IN | INS | OUT | OUTS => I,

            POPR | PUSHR => N,

            XCHG | XCHGS => X,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    None,
    Constant(u32),
    Register(u8),
    IndirectConstant(u8, u32),
    IndirectRegister(u8, u8, u8, u32)
}

impl Operand {
    fn is_reg_mem(&self) -> bool {
        match self {
            &Operand::Register(_) |
            &Operand::IndirectConstant(..) |
            &Operand::IndirectRegister(..) => true,
            _ => false
        }
    }

    fn is_const(&self) -> bool {
        match self {
            &Operand::Constant(_) => true,
            _ => false
        }
    }
}

pub trait OperandParse {
    fn decode_operands(&mut self, operation: Operation) -> (Operand, Operand);
    fn read_operand(&mut self) -> Operand;
    fn read_operand_const(&mut self, sz: u8) -> u32;
}

impl OperandParse for Cpu {
    fn decode_operands(&mut self, operation: Operation) -> (Operand, Operand) {
        match Operation::prototype(operation) {
            Prototype::N => {
                (Operand::None, Operand::None)
            }
            Prototype::A => {
                let l = self.read_operand();
                let r = self.read_operand();

                if !r.is_reg_mem() {
                    self.instr_interrupt = true;
                }

                debug!("L: {}, R: {}", l, r);

                (l, r)
            }
            Prototype::X => {
                let l = self.read_operand();
                let r = self.read_operand();

                if !r.is_reg_mem() || !l.is_reg_mem() {
                    self.instr_interrupt = true;
                }

                (l, r)
            }
            Prototype::I => {
                let l = self.read_operand();
                let r =  self.read_operand();

                if !r.is_reg_mem() || !l.is_const() {
                    self.instr_interrupt = true;
                }

                (l, r)
            }
            Prototype::P => {
                let o = self.read_operand();

                if !o.is_reg_mem() {
                    self.instr_interrupt = true;
                }

                (o, Operand::None)
            }
            Prototype::U => {
                let o = self.read_operand();
                (o, Operand::None)
            }
            Prototype::T => {
                let imm = self.read_operand_const(1);

                (Operand::Constant(imm), Operand::None)
            }
        }
    }

    fn read_operand(&mut self) -> Operand {
        let rp = self.rp;
        let descriptor = self.mem_get_short(rp);
        self.rp.wrapping_increment(1);

        debug!("Reading operand descriptor: {} as {:b}", descriptor, descriptor);

        if descriptor & 0b1 == 0 {
            // DIRECT
            if descriptor & 0b10 == 0 {
                // CONSTANT
                //TODO: introduce variables for these
                let const_sz = (descriptor >> 2) & 0b11;
                Operand::Constant(self.read_operand_const(const_sz))
            } else {
                // REGISTER
                let register = (descriptor >> 2) & 0b1111;
                Operand::Register(register)
            }
        } else {
            // INDIRECT
            if descriptor & 0b10 == 0 {
                // CONSTANT BASE
                let const_sz = (descriptor >> 2) & 0b11;
                let register = (descriptor >> 4) & 0b1111;
                Operand::IndirectConstant(register, self.read_operand_const(const_sz))
            } else {
                // REGISTER BASE
                let rp = self.rp;
                let descriptor2 = self.mem_get_short(rp);
                self.rp.wrapping_increment(1);

                let scale = (descriptor >> 2) & 0b11;
                let base_register = (descriptor >> 4) & 0b1111;
                let offset_register = descriptor2 & 0b1111;
                let const_sz = (descriptor2 >> 4) & 0b11;
                Operand::IndirectRegister(base_register,
                                          offset_register, scale,
                                          self.read_operand_const(const_sz))
            }
        }
    }

    fn read_operand_const(&mut self, sz: u8) -> u32 {
        debug!("Reading operand const size: {}", sz);
        let rp = self.rp;
        match sz {
            0 => 0, // 0-byte constant
            1 => { // 1-byte constant
                let constant = self.mem_get_short(rp);
                self.rp.wrapping_increment(1);
                constant as u32
            }
            2 => { // 2-byte constant
                let constant = self.mem_get_short(rp) as u32 |
                               self.mem_get_short(rp.wrapping_add(1)) as u32 >> 8;
                self.rp.wrapping_increment(2);
                constant
            }
            3 => { // 4-byte constant
                let constant = self.mem_get_long(rp);
                self.rp.wrapping_increment(4);
                constant
            }
            _ => unreachable!()
        }
    }
}

pub trait OperandCompute {
    fn get_op_long(&mut self, op: Operand) -> Option<u32>;
    fn get_ops_long(&mut self, op1: Operand, op2: Operand) -> Option<(u32, u32)>;
    fn store_op_long(&mut self, op: Operand, val: u32) -> bool;
    fn get_op_short(&mut self, op: Operand) -> Option<u8>;
    fn get_ops_short(&mut self, op1: Operand, op2: Operand) -> Option<(u8, u8)>;
    fn store_op_short(&mut self, op: Operand, val: u8) -> bool;
}

impl OperandCompute for Cpu {
    fn get_op_long(&mut self, op: Operand) -> Option<u32> {
        let val = match op {
            Operand::None => unreachable!(),
            Operand::Constant(c) => c,
            Operand::Register(r) => self.reg[r as usize],
            Operand::IndirectConstant(r, c) => {
                let addr = self.reg[r as usize].wrapping_add(c);
                self.mem_get_long(addr)
            }
            Operand::IndirectRegister(b, o, s, c) => {
                let addr = (self.reg[o as usize] << s).wrapping_add(self.reg[b as usize])
                                                      .wrapping_add(c);
                self.mem_get_long(addr)
            }
        };

        if !self.has_memory_interrupt() {
            Some(val)
        } else {
            None
        }
    }

    fn get_ops_long(&mut self, op1: Operand, op2: Operand) -> Option<(u32, u32)> {
        if let Some(val1) = self.get_op_long(op1) {
            if let Some(val2) = self.get_op_long(op2) {
                return Some((val1, val2));
            }
        }

        None
    }

    fn store_op_long(&mut self, op: Operand, val: u32) -> bool {
        match op {
            Operand::None => unreachable!(),
            Operand::Constant(_) => unreachable!(),
            Operand::Register(r) => {
                self.reg[r as usize] = val;
            }
            Operand::IndirectConstant(r, c) => {
                let addr = self.reg[r as usize].wrapping_add(c);
                self.mem_set_long(addr, val);
            }
            Operand::IndirectRegister(b, o, s, c) => {
                let addr = (self.reg[o as usize] << s).wrapping_add(self.reg[b as usize])
                                                      .wrapping_add(c);
                self.mem_set_long(addr, val);
            }
        }

        !self.has_memory_interrupt()
    }

    fn get_op_short(&mut self, op: Operand) -> Option<u8> {
        let val = match op {
            Operand::None => unreachable!(),
            Operand::Constant(c) => {
                if c > 0xFF {
                    return None;
                }

                c as u8
             },
            Operand::Register(r) => (self.reg[(r >> 2) as usize] >> (8 * (r & 0b11))) as u8,
            Operand::IndirectConstant(r, c) => {
                let addr = self.reg[r as usize].wrapping_add(c);
                self.mem_get_short(addr)
            }
            Operand::IndirectRegister(b, o, s, c) => {
                let addr = (self.reg[o as usize] << s).wrapping_add(self.reg[b as usize])
                                                      .wrapping_add(c);
                self.mem_get_short(addr)
            }
        };

        if !self.has_memory_interrupt() {
            Some(val)
        } else {
            None
        }
    }

    fn get_ops_short(&mut self, op1: Operand, op2: Operand) -> Option<(u8, u8)> {
        if let Some(val1) = self.get_op_short(op1) {
            if let Some(val2) = self.get_op_short(op2) {
                return Some((val1, val2));
            }
        }

        None
    }

    fn store_op_short(&mut self, op: Operand, val: u8) -> bool {
        match op {
            Operand::None => unreachable!(),
            Operand::Constant(_) => unreachable!(),
            Operand::Register(r) => {
                let reg = r >> 2;
                let sel = r & 0b11;
                let mask = 0xFF << (sel * 8);
                self.reg[reg as usize] &= !mask;
                self.reg[reg as usize] |= (val as u32) << (sel * 8);
            }
            Operand::IndirectConstant(r, c) => {
                let addr = self.reg[r as usize].wrapping_add(c);
                self.mem_set_short(addr, val);
            }
            Operand::IndirectRegister(b, o, s, c) => {
                let addr = (self.reg[o as usize] << s).wrapping_add(self.reg[b as usize])
                                                      .wrapping_add(c);
                self.mem_set_short(addr, val);
            }
        }

        !self.has_memory_interrupt()
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operation::*;

        match self {
            &ADD => write!(f, "ADD"),
            &ADDS => write!(f, "ADDS"),
            &SUB => write!(f, "SUB"),
            &SUBS => write!(f, "SUBS"),
            &ADC => write!(f, "ADC"),
            &ADCS => write!(f, "ADCS"),
            &SBB => write!(f, "SBB"),
            &SBBS => write!(f, "SBBS"),
            &RSUB => write!(f, "RSUB"),
            &RSUBS => write!(f, "RSUBS"),
            &NOR => write!(f, "NOR"),
            &NORS => write!(f, "NORS"),
            &NAND => write!(f, "NAND"),
            &NANDS => write!(f, "NANDS"),
            &OR => write!(f, "OR"),
            &ORS => write!(f, "ORS"),
            &ORN => write!(f, "ORN"),
            &ORNS => write!(f, "ORNS"),
            &AND => write!(f, "AND"),
            &ANDS => write!(f, "ANDS"),
            &ANDN => write!(f, "ANDN"),
            &ANDNS => write!(f, "ANDNS"),
            &XNOR => write!(f, "XNOR"),
            &XNORS => write!(f, "XNORS"),
            &NOT => write!(f, "NOT"),
            &NOTS => write!(f, "NOTS"),
            &XOR => write!(f, "XOR"),
            &XORS => write!(f, "XORS"),
            &CMP => write!(f, "CMP"),
            &CMPS => write!(f, "CMPS"),
            &TEST => write!(f, "TEST"),
            &TESTS => write!(f, "TESTS"),
            &JMP => write!(f, "JMP"),
            &JE => write!(f, "JE"),
            &JNE => write!(f, "JNE"),
            &JL => write!(f, "JL"),
            &JLE => write!(f, "JLE"),
            &JG => write!(f, "JG"),
            &JGE => write!(f, "JGE"),
            &JLU => write!(f, "JLU"),
            &JLEU => write!(f, "JLEU"),
            &JGU => write!(f, "JGU"),
            &JGEU => write!(f, "JGEU"),
            &CALL => write!(f, "CALL"),
            &RET => write!(f, "RET"),
            &HLT => write!(f, "HLT"),
            &INT => write!(f, "INT"),
            &IRET => write!(f, "IRET"),
            &LOM => write!(f, "LOM"),
            &ROM => write!(f, "ROM"),
            &LOI => write!(f, "LOI"),
            &ROI => write!(f, "ROI"),
            &ROP => write!(f, "ROP"),
            &LFL => write!(f, "LFL"),
            &RFL => write!(f, "RFL"),
            &LOT => write!(f, "LOT"),
            &ROT => write!(f, "ROT"),
            &LOS => write!(f, "LOS"),
            &ROS => write!(f, "ROS"),
            &LOF => write!(f, "LOF"),
            &ROF => write!(f, "ROF"),
            &MOV => write!(f, "MOV"),
            &MOVS => write!(f, "MOVS"),
            &POP => write!(f, "POP"),
            &POPS => write!(f, "POPS"),
            &PUSH => write!(f, "PUSH"),
            &PUSHS => write!(f, "PUSHS"),
            &POPR => write!(f, "POPR"),
            &PUSHR => write!(f, "PUSHR"),
            &IN => write!(f, "IN"),
            &INS => write!(f, "INS"),
            &OUT => write!(f, "OUT"),
            &OUTS => write!(f, "OUTS"),
            &XCHG => write!(f, "XCHG"),
            &XCHGS => write!(f, "XCHGS"),
            &MOVE => write!(f, "MOVE"),
            &MOVNE => write!(f, "MOVNE"),
            &MOVL => write!(f, "MOVL"),
            &MOVLE => write!(f, "MOVLE"),
            &MOVG => write!(f, "MOVG"),
            &MOVGE => write!(f, "MOVGE"),
            &MOVLU => write!(f, "MOVLU"),
            &MOVLEU => write!(f, "MOVLEU"),
            &MOVGU => write!(f, "MOVGU"),
            &MOVGEU => write!(f, "MOVGEU"),
            &MOVES => write!(f, "MOVES"),
            &MOVNES => write!(f, "MOVNES"),
            &MOVLS => write!(f, "MOVLS"),
            &MOVLES => write!(f, "MOVLES"),
            &MOVGS => write!(f, "MOVGS"),
            &MOVGES => write!(f, "MOVGES"),
            &MOVLUS => write!(f, "MOVLUS"),
            &MOVLEUS => write!(f, "MOVLEUS"),
            &MOVGUS => write!(f, "MOVGUS"),
            &MOVGEUS => write!(f, "MOVGEUS")
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operand::None => Ok(()),
            &Operand::Constant(c) => write!(f, "const {}", c),
            &Operand::Register(r) => write!(f, "r{}", r),
            &Operand::IndirectConstant(r, c) => write!(f, "[r{} + {}]", r, c),
            &Operand::IndirectRegister(b, o, s, c) => write!(f, "[r{} + {} * r{} + {}]", b, 1 << s, o, c),
        }
    }
}
