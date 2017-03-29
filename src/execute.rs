use debug::*;
use wrapping_util::WrappingIncrement;

use cpu::Cpu;
use mem::Mem;
use operation::{Operation, Operand, OperandCompute};
use flag::*;
use interrupt::*;

pub trait Execute {
    fn execute_operation(&mut self, operation: Operation, op1: Operand, op2: Operand);
}

impl Execute for Cpu {
    fn execute_operation(&mut self, operation: Operation, op1: Operand, op2: Operand) {
        use operation::Operation::*;

        debug!("{} {} {}", operation, op1, op2);

        match operation {
            ADD => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a as u64 + b as u64;

                    if self.store_op_long(op2, c as u32) {
                        self.set_arith_long(a, b, c);
                    }
                }
            },
            ADDS => {
                /*if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a as u32 + b as u32;

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }*/
            },
            SUB => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = (b as u64).wrapping_sub(a as u64);

                    if self.store_op_long(op2, c as u32) {
                        self.set_arith_long(a, b, c);
                    }
                }
            },
            SUBS => {
                /*if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = (b as u32).wrapping_sub(a as u32);

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }*/
            },
            ADC => {},
            ADCS => {},
            SBB => {},
            SBBS => {},
            NOR => {},
            NORS => {},
            NAND => {},
            NANDS => {},
            OR => {},
            ORS => {},
            ORN => {},
            ORNS => {},
            AND => {},
            ANDS => {},
            ANDN => {},
            ANDNS => {},
            XNOR => {},
            XNORS => {},
            NOT => {},
            NOTS => {},
            XOR => {},
            XORS => {},
            CMP => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = (b as u64).wrapping_sub(a as u64);

                    if self.store_op_long(op2, c as u32) {
                        self.set_arith_long(a, b, c);
                    }
                }
            },
            CMPS => {},
            TEST => {},
            TESTS => {},
            JMP => {
                if let Some(val) = self.get_op_long(op1) {
                    self.rp = val;
                }
            },
            JE => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JNE => {
                if let Some(val) = self.get_op_long(op1) {
                    if !self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JL => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JLE => {
                if let Some(val) = self.get_op_long(op1) {
                    if (self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            || self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JG => {
                if let Some(val) = self.get_op_long(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            && !self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JGE => {
                if let Some(val) = self.get_op_long(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG)) {
                        self.rp = val;
                    }
                }
            },
            JLU => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(CARRY_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JLEU => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(CARRY_FLAG) || self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JGU => {
                if let Some(val) = self.get_op_long(op1) {
                    if !self.flag_get(CARRY_FLAG) && !self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JGEU => {
                if let Some(val) = self.get_op_long(op1) {
                    if !self.flag_get(CARRY_FLAG) {
                        self.rp = val;
                    }
                }
            },
            CALL => {},
            RET => {},
            HLT => {
                if self.flag_get(PROTECT_FLAG) {
                    self.interrupt_queue.push_back(HALT_INTERRUPT);
                } else {
                    fatal!("Halt instruction reached!");
                }
            },
            INT => {},
            IRET => {},
            LOM => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    if let Some(val) = self.get_op_long(op1) {
                        self.rm = val;
                    }
                }
            },
            ROM => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    let val = self.rm;
                    self.store_op_long(op1, val);
                }
            },
            LOI => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    if let Some(val) = self.get_op_long(op1) {
                        self.ri = val;
                    }
                }
            },
            ROI => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    let val = self.ri;
                    self.store_op_long(op1, val);
                }
            },
            ROP => {
                let rp = self.rp;
                self.store_op_long(op1, rp);
            },
            LFL => {},
            RFL => {
                let rflags = self.rflags;
                self.store_op_long(op1, rflags);
            },
            LOT => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    if let Some(val) = self.get_op_long(op1) {
                        self.rkt = val;
                    }
                }
            },
            ROT => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    let val = self.rkt;
                    self.store_op_long(op1, val);
                }
            },
            LOS => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    if let Some(val) = self.get_op_long(op1) {
                        self.rks = val;
                    }
                }
            },
            ROS => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    let val = self.rks;
                    self.store_op_long(op1, val);
                }
            },
            LOF => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    if let Some(val) = self.get_op_long(op1) {
                        self.rf = val;
                    }
                }
            },
            ROF => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    let val = self.rf;
                    self.store_op_long(op1, val);
                }
            },
            MOV => {
                if let Some(val) = self.get_op_long(op1) {
                    self.store_op_long(op2, val);
                }
            },
            MOVS => {},
            POP => {
                let rs = self.reg[15];
                let val = self.mem_get_long(rs);

                if self.store_op_long(op1, val) {
                    self.reg[15].wrapping_increment(4);
                }
            },
            POPS => {},
            PUSH => {
                let rs = self.reg[15].wrapping_sub(4);

                if let Some(val) = self.get_op_long(op1) {
                    self.mem_set_long(rs, val);

                    if !self.has_memory_interrupt() {
                        self.reg[15] = rs;
                    }
                }
            },
            PUSHS => {},
            IN => {},
            INS => {},
            OUT => {},
            OUTS => {},
            XCHG => {
                if let Some((val1, val2)) = self.get_ops_long(op1, op2) {
                    self.store_op_long(op1, val2) && self.store_op_long(op2, val1);
                }
            },
            XCHGS => {},
            MOVE => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(ZERO_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVNE => {
                if let Some(val) = self.get_op_long(op1) {
                    if !self.flag_get(ZERO_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVL => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVLE => {
                if let Some(val) = self.get_op_long(op1) {
                    if (self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            || self.flag_get(ZERO_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVG => {
                if let Some(val) = self.get_op_long(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            && !self.flag_get(ZERO_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVGE => {
                if let Some(val) = self.get_op_long(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG)) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVLU => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(CARRY_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVLEU => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(CARRY_FLAG) || self.flag_get(ZERO_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVGU => {
                if let Some(val) = self.get_op_long(op1) {
                    if !self.flag_get(CARRY_FLAG) && !self.flag_get(ZERO_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            },
            MOVGEU => {
                if let Some(val) = self.get_op_long(op1) {
                    if !self.flag_get(CARRY_FLAG) {
                        self.store_op_long(op2, val);
                    }
                }
            }
            _ => {}
        }
    }
}
