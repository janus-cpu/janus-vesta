use std::fmt::{self, Formatter, Display};
use cpu::Registers;
use instruction::{Instruction, Operation, Operand, Size};

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let size = match self.size {
            Size::Short => ".s",
            Size::Long => ""
        };
        write!(f, "INSRUCTION: {}{} {}, {}", self.op, size, self.op1, self.op2)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Operand::None => { write!(f, "") },
            Operand::Reg(r) => { write!(f, "r{}", r) },
            Operand::RegDeref(r) => { write!(f, "[r{}]", r) },
            Operand::RegOff(r, o) => { write!(f, "[r{} OFF {}]", r, o) },
            Operand::Const => { write!(f, "CONST") }
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
         let opstring = match *self {
             Operation::NOP => "NOP",
             Operation::ADD => "ADD",
             Operation::ADC => "ADC",
             Operation::SUB => "SUB",
             Operation::SBB => "SBB",
             Operation::CMP1 => "CMP1",
             Operation::CMP2 => "CMP2",
             Operation::TEST1 => "TEST1",
             Operation::TEST2 => "TEST2",
             Operation::DEC => "DEC",
             Operation::INC => "INC",
             //0xB => Operation::UDIV,
             //0xC => Operation::UMUL,
             //0xD => Operation::SDIV,
             //0xE => Operation::SMUL,
             Operation::NEG => "NEG",
             Operation::NOT => "NOT",
             Operation::AND => "AND",
             Operation::OR => "OR",
             Operation::XOR => "XOR",

             Operation::JMP => "JMP",
             Operation::JE => "JE",
             Operation::JNE => "JNE",
             Operation::JL => "JL",
             Operation::JLE => "JLE",
             Operation::JG => "JG",
             Operation::JGE => "JGE",
             Operation::JLU => "JLU",
             Operation::JLEU => "JLEU",
             Operation::JGU => "JGU",
             Operation::JGEU => "JGEU",
             Operation::CALL => "CALL",
             Operation::RET => "RET",
             Operation::HLT => "HLT",

             Operation::LOM => "LOM",
             Operation::ROM => "ROM",
             Operation::LOI => "LOI",
             Operation::ROI => "ROI",
             Operation::ROP => "ROP",
             Operation::LFL => "LFL",
             Operation::RFL => "RFL",

             Operation::MOV => "MOV",
             Operation::POP => "POP",
             Operation::PUSH => "PUSH",
             Operation::IN => "IN",
             Operation::OUT => "OUT",
             Operation::XCHG => "XCHG",
             Operation::MOVE => "MOVE",
             Operation::MOVNE => "MOVNE",
             Operation::MOVL => "MOVL",
             Operation::MOVLE => "MOVLE",
             Operation::MOVG => "MOVG",
             Operation::MOVGE => "MOVGE",
             Operation::MOVLU => "MOVLU",
             Operation::MOVLEU => "MOVLEU",
             Operation::MOVGU => "MOVGU",
             Operation::MOVGEU => "MOVGEU"
         };

         write!(f, "{}", opstring)
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "\n"));

        for i in 0..15 {
            try!(write!(f, "r{}: {}, ", i, self.gp[i]));
            if i % 4 == 0 && i != 0 {
                try!(write!(f, "\n"));
            }
        }

        try!(write!(f, "rr: {}", self.rr));

        for i in 0..6 {
            try!(write!(f, "re{}: {}, ", i, self.re[i]));
            if i % 4 == 0 && i != 0 {
                try!(write!(f, "\n"));
            }
        }

        for i in 0..7 {
            try!(write!(f, "rk{}: {}, ", i, self.gp[i]));
            if i % 4 == 0 && i != 0 {
                try!(write!(f, "\n"));
            }
        }

        try!(write!(f, "rm: {}", self.rm));
        write!(f, "ri: {}", self.ri)
    }
}
