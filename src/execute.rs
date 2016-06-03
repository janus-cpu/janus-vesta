use instruction::Instruction;
use instruction::Size;
use instruction::Operation;
use cpu::Cpu;
use cpu::Fault;
use cpu::FlagOperations;
use die::die;

pub trait Instructor {
    fn instruct(&mut self, instruction: Instruction);
    fn instruct_short(&mut self, instruction: Instruction);
    fn instruct_long(&mut self, instruction: Instruction);
}

impl Instructor for Cpu {
    fn instruct(&mut self, inst: Instruction) {
        match inst.size {
            Size::Short => self.instruct_short(inst),
            Size::Long  => self.instruct_long(inst)
        }
    }

    #[allow(non_snake_case)]
    fn instruct_short(&mut self, inst: Instruction) {
        let A = self.retrieve_op_short(inst.op1) as u32;
        let B = self.retrieve_op_short(inst.op2) as u32;

        match inst.op {
            Operation::NOP => { },
            Operation::ADD => {

            },
            Operation::ADC => {

            },
            Operation::SUB => {

            },
            Operation::SBB => {

            },
            Operation::CMP1 => {

            },
            Operation::CMP2 => {

            },
            Operation::TEST1 => {

            },
            Operation::TEST2 => {

            },
            Operation::DEC => {

            },
            Operation::INC => {

            },
            Operation::NEG => {

            },
            Operation::NOT => {
                let C = !A;

                self.set_carry_flag(false);
                self.set_ovf_flag(false);
                self.set_zero_flag((C & 0xFF) == 0);
                self.set_neg_flag(C & 0x80);
                self.store_op_short(inst.op1, (C & 0xFF) as u8);
            },
            Operation::AND => {
                let C = A & B;

                self.set_carry_flag(false);
                self.set_ovf_flag(false);
                self.set_zero_flag((C & 0xFF) == 0);
                self.set_neg_flag(C & 0x80);
                self.store_op_short(inst.op2, (C & 0xFF) as u8);
            },
            Operation::OR => {
                let C = A & B;

                self.set_carry_flag(false);
                self.set_ovf_flag(false);
                self.set_zero_flag((C & 0xFF) == 0);
                self.set_neg_flag(C & 0x80);
                self.store_op_short(inst.op2, (C & 0xFF) as u8);
            },
            Operation::XOR => {
                let C = A ^ B;

                self.set_carry_flag(false);
                self.set_ovf_flag(false);
                self.set_zero_flag((C & 0xFF) == 0);
                self.set_neg_flag(C & 0x80);
                self.store_op_short(inst.op2, (C & 0xFF) as u8);
            },
            Operation::MOV => {
                self.store_op_short(inst.op2, A as u8);
            },
            Operation::POP => {
                let mut stack = self.registers.gp[15];
                let val = self.retrieve_mem_short(stack);
                self.store_op_short(inst.op1, val);
                stack = stack.wrapping_add(4);
                self.registers.gp[15] = stack;
            },
            Operation::PUSH => {
                let mut stack = self.registers.gp[15];
                stack = stack.wrapping_sub(1);
                self.registers.gp[15] = stack;
                self.store_mem_short(stack, A as u8);
            },
            Operation::IN => {

            },
            Operation::OUT => {
                println!("Printed {} ({}) to CPU out {}", B, B as u8 as char, A);
            },
            Operation::XCHG => {
                self.store_op_short(inst.op1, B as u8);
                self.store_op_short(inst.op2, A as u8);
            },
            Operation::MOVE => {
                if self.get_zero_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVNE => {
                if !self.get_zero_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVL => {
                if self.get_neg_flag() != self.get_ovf_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVLE => {
                if self.get_neg_flag() != self.get_ovf_flag()
                   || self.get_zero_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVG => {
                if self.get_neg_flag() == self.get_ovf_flag()
                   && self.get_zero_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVGE => {
                if self.get_neg_flag() == self.get_ovf_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVLU => {
                if self.get_carry_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVLEU => {
                if self.get_carry_flag() || self.get_zero_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVGU => {
                if !self.get_carry_flag() && !self.get_zero_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            },
            Operation::MOVGEU => {
                if !self.get_carry_flag() {
                    self.store_op_short(inst.op2, A as u8);
                }
            }
            _ => {
                self.fault(Fault::FAULT_ILLEGAL_INSTRUCTION);
            }
        }
    }

    #[allow(non_snake_case)]
    fn instruct_long(&mut self, inst: Instruction) {
        let A = self.retrieve_op_long(inst.op1);
        let B = self.retrieve_op_long(inst.op2);

        match inst.op {
            Operation::NOP => { },
            Operation::ADD => {
                let C = A.wrapping_add(B);
                debug!("Add {} + {} got {}", A, B, C);
                let lo = (A & 0xFFFF) + (B & 0xFFFF);
                let hi = (lo >> 16) + (A >> 16) + (B >> 16);
                self.set_carry_flag(hi & 0x10000);
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let carrychain = (A & B) | ((!C) & (A | B));
                self.set_ovf_flag(XOR2(carrychain >> 30));
                self.store_op_long(inst.op2, C);
            },
            Operation::ADC => {
                let carryin = if self.get_carry_flag() { 1 } else { 0 };
                let C = A.wrapping_add(B).wrapping_add(carryin);
                let lo = (A & 0xFFFF) + (B & 0xFFFF) + carryin;
                let hi = (lo >> 16) + (A >> 16) + (B >> 16);
                self.set_carry_flag(hi & 0x10000);
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let carrychain = (A & B) | ((!C) & (A | B));
                self.set_ovf_flag(XOR2(carrychain >> 30));
                self.store_op_long(inst.op2, C);
            },
            Operation::SUB => {
                let C = B.wrapping_sub(A);

                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let borrowchain = (C & ((!B) | A)) | ((!B) & A);
                self.set_carry_flag(borrowchain & 0x80000000);
                self.set_ovf_flag(XOR2(borrowchain >> 30));
                self.store_op_long(inst.op2, C);
            },
            Operation::SBB => {
                let carryin = if self.get_carry_flag() { 1 } else { 0 };
                let C = B.wrapping_sub(A).wrapping_sub(carryin);

                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let borrowchain = (C & ((!B) | A)) | ((!B) & A);
                self.set_carry_flag(borrowchain & 0x80000000);
                self.set_ovf_flag(XOR2(borrowchain >> 30));
                self.store_op_long(inst.op2, C);
            },
            Operation::CMP1 | Operation::CMP2 => {
                let C = B.wrapping_sub(A);
                debug!("Comparing {} - {}, got {}", B, A, C);

                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let borrowchain = (C & ((!B) | A)) | ((!B) & A);
                self.set_carry_flag(borrowchain & 0x80000000);
                self.set_ovf_flag(XOR2(borrowchain >> 30));
            },
            Operation::TEST1 | Operation::TEST2 => {
                let C = A & B;

                self.set_carry_flag(false); // clear
                self.set_ovf_flag(false);   // clear
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);
            },
            Operation::DEC => {
                let C = B.wrapping_sub(1);

                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let borrowchain = (C & ((!B) | 1)) | ((!B) & 1);
                self.set_carry_flag(borrowchain & 0x80000000);
                self.set_ovf_flag(XOR2(borrowchain >> 30));
                self.store_op_long(inst.op1, C);
            },
            Operation::INC => {
                let C = A.wrapping_add(1);
                let lo = (A & 0xFFFF) + 1;
                let hi = (lo >> 16) + (A >> 16);
                self.set_carry_flag(hi & 0x10000);
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                let carrychain = (A & 1) | ((!C) & (1 | B));
                self.set_ovf_flag(XOR2(carrychain >> 30));
                self.store_op_long(inst.op1, C);
            },
            Operation::NEG => {
                let C = (0 as u32).wrapping_sub(A);

                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);

                self.set_carry_flag(false);
                self.set_ovf_flag(false); //is no overflow a shortcoming? TODO
                self.store_op_long(inst.op1, C);
            },
            Operation::NOT => {
                self.set_carry_flag(false); // clear
                self.set_ovf_flag(false);   // clear
                self.set_zero_flag(A == 0);
                self.set_neg_flag((!A) & 0x80000000);
                self.store_op_long(inst.op1, !A);
            },
            Operation::AND => {
                let C = A & B;

                self.set_carry_flag(false); // clear
                self.set_ovf_flag(false);   // clear
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);
                self.store_op_long(inst.op2, C);
            },
            Operation::OR => {
                let C = A | B;

                self.set_carry_flag(false); // clear
                self.set_ovf_flag(false);   // clear
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);
                self.store_op_long(inst.op2, C);
            },
            Operation::XOR => {
                let C = A ^ B;

                self.set_carry_flag(false); // clear
                self.set_ovf_flag(false);   // clear
                self.set_zero_flag(C == 0);
                self.set_neg_flag(C & 0x80000000);
                self.store_op_long(inst.op2, C);
            },
            Operation::JMP => {
                self.pc = A;
            },
            Operation::JE => {
                if self.get_zero_flag() {
                    self.pc = A;
                }
            },
            Operation::JNE => {
                if !self.get_zero_flag() {
                    self.pc = A;
                }
            },
            Operation::JL => {
                if self.get_neg_flag() != self.get_ovf_flag() {
                    self.pc = A;
                }
            },
            Operation::JLE => {
                if self.get_neg_flag() != self.get_ovf_flag()
                   || self.get_zero_flag() {
                    self.pc = A;
                }
            },
            Operation::JG => {
                if self.get_neg_flag() == self.get_ovf_flag()
                   && !self.get_zero_flag() {
                    self.pc = A;
                }
            },
            Operation::JGE => {
                if self.get_neg_flag() == self.get_ovf_flag() {
                    self.pc = A;
                }
            },
            Operation::JLU => {
                if self.get_carry_flag() {
                    self.pc = A;
                }
            },
            Operation::JLEU => {
                if self.get_carry_flag() || self.get_zero_flag() {
                    self.pc = A;
                }
            },
            Operation::JGU => {
                if !self.get_carry_flag() && !self.get_zero_flag() {
                    self.pc = A;
                }
            },
            Operation::JGEU => {
                if !self.get_carry_flag() {
                    self.pc = A;
                }
            },
            Operation::CALL => {
                // decrement stack
                let mut stack = self.registers.gp[15];
                stack = stack.wrapping_sub(4);
                self.registers.gp[15] = stack;
                // Push old rr
                let oldrr = self.registers.rr;
                self.store_mem_long(stack, oldrr);
                // move pc into new rr
                self.registers.rr = self.pc;
                // jump to new location
                self.pc = A;
            },
            Operation::RET => {
                // jump to rr
                self.pc = self.registers.rr;
                // pop old rr
                let stack = self.registers.gp[15];
                let val = self.retrieve_mem_long(stack);
                self.registers.rr = val;
                // increment stack
                let mut stack = self.registers.gp[15];
                stack = stack.wrapping_add(4);
                self.registers.gp[15] = stack;
            },
            Operation::HLT => {
                self.fault(Fault::FAULT_HALT);
                die("Halted!");
            },
            Operation::ROM => {
                let val = self.registers.rm;
                self.store_op_long(inst.op1, val);
            },
            Operation::LOM => {
                self.registers.rm = A;
            },
            Operation::ROI => {
                let val = self.registers.ri;
                self.store_op_long(inst.op1, val);
            },
            Operation::LOI => {
                self.registers.ri = A;
            },
            Operation::ROP => {
                let val = self.pc;
                self.store_op_long(inst.op1, val);
            },
            Operation::LFL => {
                //TODO: mask for privileges.
                self.registers.rflags = A;
            },
            Operation::RFL => {
                let val = self.registers.rflags;
                self.store_op_long(inst.op1, val);
            },
            Operation::MOV => {
                self.store_op_long(inst.op2, A);
            },
            Operation::PUSH => {
                let mut stack = self.registers.gp[15];
                stack = stack.wrapping_sub(4);
                self.registers.gp[15] = stack;
                self.store_mem_long(stack, A);
            },
            Operation::POP => {
                let mut stack = self.registers.gp[15];
                let val = self.retrieve_mem_long(stack);
                self.store_op_long(inst.op1, val);
                stack = stack.wrapping_add(4);
                self.registers.gp[15] = stack;
            },
            Operation::IN => {

            },
            Operation::OUT => {
                //TODO: do this actually...
                println!("Printed {} ({}) to CPU out {}", B, B as u8 as char, A);
            },
            Operation::XCHG => {
                self.store_op_long(inst.op2, A);
                self.store_op_long(inst.op1, B);
            },
            Operation::MOVE => {
                if self.get_zero_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVNE => {
                if !self.get_zero_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVL => {
                if self.get_neg_flag() != self.get_ovf_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVLE => {
                if self.get_neg_flag() != self.get_ovf_flag()
                   || self.get_zero_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVG => {
                if self.get_neg_flag() == self.get_ovf_flag()
                   && self.get_zero_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVGE => {
                if self.get_neg_flag() == self.get_ovf_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVLU => {
                if self.get_carry_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVLEU => {
                if self.get_carry_flag() || self.get_zero_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVGU => {
                if !self.get_carry_flag() && !self.get_zero_flag() {
                    self.store_op_long(inst.op2, A);
                }
            },
            Operation::MOVGEU => {
                if !self.get_carry_flag() {
                    self.store_op_long(inst.op2, A);
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn XOR2(a: u32) -> bool {
    let abool = (a >> 30) & 0b1 == 1;
    let bbool = (a >> 31) & 0b1 == 1;

    (abool && !bbool) || (bbool && !abool)
}
