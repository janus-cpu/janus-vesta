#[derive(Copy, Clone)]
pub struct Instruction {
    pub op: Operation,
    pub size: Size,
    pub op1: Operand,
    pub op2: Operand
}

impl Instruction {
    pub fn decode(
        opcode: u32, sizei: u32, type1: u32, type2: u32, op1i: u32, op2i: u32)
        -> Result<Instruction, ()> {
        let operation = try!(Operation::parse(opcode));

        let size = match sizei {
            0 => Size::Long,
            1 => Size::Short,
            _ => { unreachable!() }
        };

        let op1 = try!(Operand::typecheck1(operation, type1, op1i, size));

        let op2 = try!(Operand::typecheck2(operation, type2, op2i, size));

        Ok(Instruction { op: operation, size: size, op1: op1, op2: op2 })
    }

    pub fn dead() -> Instruction {
        Instruction { op: Operation::NOP, size: Size::Long,
                      op1: Operand::None, op2: Operand::None }
    }
}

#[derive(Copy, Clone)]
pub enum Size { Long, Short }

#[derive(Copy, Clone)]
pub enum Operation {
    NOP, ADD, ADC, SUB, SBB, CMP1, CMP2, TEST1, TEST2, DEC, INC,
    //UDIV, UMUL, SDIV, SMUL,
    NEG, NOT, AND, OR, XOR,
    JMP, JE, JNE, JL, JLE, JG, JGE, JLU, JLEU, JGU, JGEU, CALL, RET, HLT,
    LOM, ROM, LOI, ROI, ROP, LFL, RFL,
    MOV, POP, PUSH, IN, OUT, XCHG,
    MOVE, MOVNE, MOVL, MOVLE, MOVG, MOVGE, MOVLU, MOVLEU, MOVGU, MOVGEU
}

impl Operation {
    pub fn parse(opcode: u32) -> Result<Operation, ()> {
        let op = match opcode {
            0x0 => Operation::NOP,
            0x1 => Operation::ADD,
            0x2 => Operation::ADC,
            0x3 => Operation::SUB,
            0x4 => Operation::SBB,
            0x5 => Operation::CMP1,
            0x6 => Operation::CMP2,
            0x7 => Operation::TEST1,
            0x8 => Operation::TEST2,
            0x9 => Operation::DEC,
            0xA => Operation::INC,
            //0xB => Operation::UDIV,
            //0xC => Operation::UMUL,
            //0xD => Operation::SDIV,
            //0xE => Operation::SMUL,
            0xF => Operation::NEG,
            0x10 => Operation::NOT,
            0x11 => Operation::AND,
            0x12 => Operation::OR,
            0x13 => Operation::XOR,

            0x30 => Operation::JMP,
            0x31 => Operation::JE,
            0x32 => Operation::JNE,
            0x33 => Operation::JL,
            0x34 => Operation::JLE,
            0x35 => Operation::JG,
            0x36 => Operation::JGE,
            0x37 => Operation::JLU,
            0x38 => Operation::JLEU,
            0x39 => Operation::JGU,
            0x3A => Operation::JGEU,
            0x3C => Operation::CALL,
            0x3D => Operation::RET,
            0x3E => Operation::HLT,

            0x40 => Operation::LOM,
            0x41 => Operation::ROM,
            0x42 => Operation::LOI,
            0x43 => Operation::ROI,
            0x44 => Operation::ROP,
            0x45 => Operation::LFL,
            0x46 => Operation::RFL,

            0x50 => Operation::MOV,
            0x51 => Operation::POP,
            0x52 => Operation::PUSH,
            0x53 => Operation::IN,
            0x54 => Operation::OUT,
            0x55 => Operation::XCHG,
            0x56 => Operation::MOVE,
            0x57 => Operation::MOVNE,
            0x58 => Operation::MOVL,
            0x59 => Operation::MOVLE,
            0x5A => Operation::MOVG,
            0x5B => Operation::MOVGE,
            0x5C => Operation::MOVLU,
            0x5D => Operation::MOVLEU,
            0x5E => Operation::MOVGU,
            0x5F => Operation::MOVGEU,
            _ => { return Err(()); }
        };

        Ok(op)
    }
}

#[derive(Copy, Clone)]
pub enum Operand {
    None,
    Reg(u32),
    RegDeref(u32),
    RegOff(u32, u32),
    Const
}

impl Operand {
    fn typecheck1(op: Operation, typeint: u32, operand: u32, s: Size) -> Result<Operand, ()> {
        let operand_parsed = match op {
            //N
            Operation::NOP |
            Operation::RET |
            Operation::HLT => {
                Operand::None
            },

            Operation::ADD |
            Operation::ADC |
            Operation::SUB |
            Operation::SBB |
            Operation::CMP1 |
            Operation::TEST1 |
            Operation::AND |
            Operation::OR |
            Operation::XOR |
            Operation::MOV |
            Operation::MOVE |
            Operation::MOVNE |
            Operation::MOVL |
            Operation::MOVLE |
            Operation::MOVG |
            Operation::MOVGE |
            Operation::MOVLU |
            Operation::MOVLEU |
            Operation::MOVGU |
            Operation::MOVGEU => { //TODO: comment these sections
                Operand::parse(typeint, operand, s)
            },

            Operation::CMP2 |
            Operation::TEST2 => {
                if typeint == 3 {
                    return Err(());
                }

                Operand::parse(typeint, operand, s)
            },

            Operation::XCHG => {
                if typeint == 3 {
                    return Err(());
                }

                Operand::parse(typeint, operand, s)
            },

           Operation::IN |
           Operation::OUT => {
               if typeint != 3 {
                   return Err(());
               }

               Operand::parse(typeint, operand, s)
           },

           Operation::DEC |
           Operation::INC |
           Operation::NEG |
           Operation::NOT |
           Operation::ROM |
           Operation::ROI |
           Operation::ROP |
           Operation::RFL |
           Operation::POP => {
               if typeint == 3 {
                   return Err(());
               }

               Operand::parse(typeint, operand, s)
           },

           Operation::JMP |
           Operation::JE |
           Operation::JNE |
           Operation::JL |
           Operation::JLE |
           Operation::JG |
           Operation::JGE |
           Operation::JLU |
           Operation::JLEU |
           Operation::JGU |
           Operation::JGEU |
           Operation::CALL |
           Operation::LOM |
           Operation::LOI |
           Operation::LFL |
           Operation::PUSH => {
               Operand::parse(typeint, operand, s)
           }
        };

        Ok(operand_parsed)
    }

    fn typecheck2(op: Operation, typeint: u32, operand: u32, s: Size) -> Result<Operand, ()> {
        let operand_parsed = match op {
            //N
            Operation::NOP |
            Operation::RET |
            Operation::HLT => {
                Operand::None
            },

            Operation::ADD |
            Operation::ADC |
            Operation::SUB |
            Operation::SBB |
            Operation::CMP1 |
            Operation::TEST1 |
            Operation::AND |
            Operation::OR |
            Operation::XOR |
            Operation::MOV |
            Operation::MOVE |
            Operation::MOVNE |
            Operation::MOVL |
            Operation::MOVLE |
            Operation::MOVG |
            Operation::MOVGE |
            Operation::MOVLU |
            Operation::MOVLEU |
            Operation::MOVGU |
            Operation::MOVGEU => {
                if typeint == 3 {
                    return Err(());
                }

                Operand::parse(typeint, operand, s)
            },

           Operation::CMP2 |
           Operation::TEST2 => {
               Operand::parse(typeint, operand, s)
           },

           Operation::XCHG => {
               if typeint == 3 {
                   return Err(());
               }

               Operand::parse(typeint, operand, s)
           },

           Operation::IN |
           Operation::OUT => {
               if typeint == 3 {
                   return Err(());
               }

               Operand::parse(typeint, operand, s)
           },

           Operation::DEC |
           Operation::INC |
           Operation::NEG |
           Operation::NOT |
           Operation::ROM |
           Operation::ROI |
           Operation::ROP |
           Operation::RFL |
           Operation::POP => {
               Operand::None
           },

           Operation::JMP |
           Operation::JE |
           Operation::JNE |
           Operation::JL |
           Operation::JLE |
           Operation::JG |
           Operation::JGE |
           Operation::JLU |
           Operation::JLEU |
           Operation::JGU |
           Operation::JGEU |
           Operation::CALL |
           Operation::LOM |
           Operation::LOI |
           Operation::LFL |
           Operation::PUSH => {
               Operand::None
           }
       };

        Ok(operand_parsed)
    }

    fn parse(typeint: u32, operand: u32, size: Size) -> Operand {
        if typeint == 0 {
            match size {
                Size::Short => {
                    Operand::Reg(operand & 0b111111)
                },
                Size::Long => {
                    Operand::Reg(operand & 0b11111)
                }
            }
        } else if typeint == 1 {
            let reg = operand & 0b11111;
            Operand::RegDeref(reg)
        } else if typeint == 2 {
            let reg = operand & 0b1111;
            let off = (operand >> 4) & 0b111;
            Operand::RegOff(reg, off)
        } else if typeint == 3 {
            Operand::Const
        } else {
            unreachable!();
        }
    }
}
