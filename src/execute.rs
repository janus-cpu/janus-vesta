use wrapping_util::WrappingIncrement;

use cpu::Cpu;
use mem::Mem;
use operation::{Operation, Operand, OperandCompute, OffsetType};
use flag::*;
use interrupt::*;

pub trait Execute {
    fn execute_operation(&mut self, operation: Operation, op1: Operand, op2: Operand);
    fn get_rp_after_jmp(&mut self, op: Operand) -> Option<u32>;
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
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a as u32 + b as u32;

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }
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
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = (b as u32).wrapping_sub(a as u32);

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }
            },
            ADC => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let carry = if self.flag_get(CARRY_FLAG) { 1 } else { 0 };
                    let c = a as u64 + b as u64 + carry;

                    if self.store_op_long(op2, c as u32) {
                        self.set_arith_long(a, b, c);
                    }
                }
            },
            ADCS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let carry = if self.flag_get(CARRY_FLAG) { 1 } else { 0 };
                    let c = a as u32 + b as u32 + carry;

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }
            },
            SBB => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let carry = if self.flag_get(CARRY_FLAG) { 1 } else { 0 };
                    let c = (b as u64).wrapping_sub(a as u64).wrapping_sub(carry);

                    if self.store_op_long(op2, c as u32) {
                        self.set_arith_long(a, b, c);
                    }
                }
            },
            SBBS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let carry = if self.flag_get(CARRY_FLAG) { 1 } else { 0 };
                    let c = (b as u32).wrapping_sub(a as u32).wrapping_sub(carry);

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }
            },
            RSUB => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = (a as u64).wrapping_sub(b as u64);

                    if self.store_op_long(op2, c as u32) {
                        self.set_arith_long(a, b, c);
                    }
                }
            },
            RSUBS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = (a as u32).wrapping_sub(b as u32);

                    if self.store_op_short(op2, c as u8) {
                        self.set_arith_short(a, b, c);
                    }
                }
            },
            NOR => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = !(a | b);
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            NORS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = !(a | b);
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            NAND => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = !(a & b);
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            NANDS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = !(a & b);
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            OR => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a | b;
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            ORS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a | b;
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            ORN => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a | !b;
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            ORNS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a | !b;
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            AND => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a & b;
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            ANDS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a & b;
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            ANDN => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a & !b;
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            ANDNS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a & !b;
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            XNOR => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = !(a ^ b);
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            XNORS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = !(a ^ b);
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            NOT => {
                if let Some(a) = self.get_op_long(op1) {
                    let c = !a;
                    self.store_op_long(op2, c);
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, a, c as u64);
                    }
                }
            },
            NOTS => {
                if let Some(a) = self.get_op_short(op1) {
                    let c = !a;
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, a, c as u32);
                    }
                }
            },
            XOR => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a ^ b;
                    if self.store_op_long(op2, c) {
                        self.set_arith_long(a, b, c as u64);
                    }
                }
            },
            XORS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a ^ b;
                    if self.store_op_short(op2, c) {
                        self.set_arith_short(a, b, c as u32);
                    }
                }
            },
            CMP => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = (b as u64).wrapping_sub(a as u64);
                    debug!("Comparing {} and {}", a, b);
                    self.set_arith_long(a, b, c);
                }
            },
            CMPS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = (b as u32).wrapping_sub(a as u32);
                    debug!("Comparing {} and {} = {}", a, b, c);
                    self.set_arith_short(a, b, c);
                }
            },
            TEST => {
                if let Some((a, b)) = self.get_ops_long(op1, op2) {
                    let c = a & b;
                    self.set_arith_long(a, b, c as u64);
                }
            },
            TESTS => {
                if let Some((a, b)) = self.get_ops_short(op1, op2) {
                    let c = a & b;
                    self.set_arith_short(a, b, c as u32);
                }
            },
            JMP => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    self.rp = val;
                }
            },
            JE => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if self.flag_get(ZERO_FLAG) {
                        debug!("Jumped to {}", val);
                        self.rp = val;
                    }
                }
            },
            JNE => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if !self.flag_get(ZERO_FLAG) {
                        debug!("Jumped to {}", val);
                        self.rp = val;
                    }
                }
            },
            JL => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JLE => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if (self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            || self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JG => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            && !self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JGE => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG)) {
                        self.rp = val;
                    }
                }
            },
            JLU => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if self.flag_get(CARRY_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JLEU => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if self.flag_get(CARRY_FLAG) || self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JGU => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if !self.flag_get(CARRY_FLAG) && !self.flag_get(ZERO_FLAG) {
                        self.rp = val;
                    }
                }
            },
            JGEU => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    if !self.flag_get(CARRY_FLAG) {
                        self.rp = val;
                    }
                }
            },
            CALL => {
                if let Some(val) = self.get_rp_after_jmp(op1) {
                    self.reg[14] = self.rp;
                    self.rp = val;
                }
            },
            RET => {
                self.rp = self.reg[14];
            },
            HLT => {
                if self.flag_get(PROTECT_FLAG) {
                    self.interrupt_queue.push_back(HALT_INTERRUPT);
                } else {
                    fatal!("Halt instruction reached!");
                }
            },
            INT => {
                if let Some(val) = self.get_op_short(op1) {
                    debug!(">>> Int {}", val);
                    if val < 8 {
                        self.protect_interrupt = true;
                    } else {
                        self.interrupt_queue.push_back(val);
                    }
                }
            },
            IRET => {
                if self.flag_get(PROTECT_FLAG) {
                    self.protect_interrupt = true;
                } else {
                    self.rp = self.pop_stack();
                    self.rflags = self.pop_stack();

                    if self.flag_get(PROTECT_FLAG) {
                        self.reg[15] = self.pop_stack();
                    }

                    if self.has_memory_interrupt() {
                        error!("Fault while returning from interrupt.");
                    }
                }
            },
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
            LFL => {
                if let Some(val) = self.get_op_long(op1) {
                    if self.flag_get(PROTECT_FLAG) {
                        self.rflags &= !ARITH_FLAGS_MASK;
                        self.rflags |= val & ARITH_FLAGS_MASK;
                    } else {
                        self.rflags = val;
                    }
                }
            },
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
            MOVS => {
                if let Some(val) = self.get_op_short(op1) {
                    self.store_op_short(op2, val);
                }
            },
            POP => {
                let rs = self.reg[15];
                let val = self.mem_get_long(rs);

                if self.store_op_long(op1, val) {
                    self.reg[15].wrapping_increment(4);
                }
            },
            POPS => {
                let rs = self.reg[15];
                let val = self.mem_get_short(rs);

                if self.store_op_short(op1, val) {
                    self.reg[15].wrapping_increment(1);
                }
            },
            PUSH => {
                let rs = self.reg[15].wrapping_sub(4);

                if let Some(val) = self.get_op_long(op1) {
                    self.mem_set_long(rs, val);

                    if !self.has_memory_interrupt() {
                        self.reg[15] = rs;
                    }
                }
            },
            PUSHS => {
                let rs = self.reg[15].wrapping_sub(1);

                if let Some(val) = self.get_op_short(op1) {
                    self.mem_set_short(rs, val);

                    if !self.has_memory_interrupt() {
                        self.reg[15] = rs;
                    }
                }
            },
            POPR => {
                let old_rs = self.reg[15];

                for i in 4..15 {
                    let rs = self.reg[15];
                    let reg = self.reg[i];
                    self.mem_set_long(rs, reg);

                    if !self.has_memory_interrupt() {
                        self.reg[15].wrapping_increment(4);
                    } else {
                        self.reg[15] = old_rs;
                        break;
                    }
                }
            },
            PUSHR => {
                let old_rs = self.reg[15];

                for i in (4..15).rev() {
                    let rs = self.reg[15].wrapping_sub(4);
                    let reg = self.reg[i];
                    self.mem_set_long(rs, reg);

                    if !self.has_memory_interrupt() {
                        self.reg[15] = rs;
                    } else {
                        self.reg[15] = old_rs;
                        break;
                    }
                }
            }
            IN => {},
            INS => {},
            OUT => {
                if let Some((port, val)) = self.get_ops_long(op1, op2) {
                    println!("0x{:X}: {} {}", port, val as u8 as char, val);
                }
            },
            OUTS => {
                if let Some((port, val)) = self.get_ops_short(op1, op2) {
                    println!("0x{:X}: {} {}", port, val as char, val);
                }
            },
            XCHG => {
                if let Some((val1, val2)) = self.get_ops_long(op1, op2) {
                    if !(self.store_op_long(op1, val2) && self.store_op_long(op2, val1)) {
                        self.store_op_long(op1, val1);
                        self.store_op_long(op2, val2);
                    }
                }
            },
            XCHGS => {
                if let Some((val1, val2)) = self.get_ops_short(op1, op2) {
                    if !(self.store_op_short(op1, val2) && self.store_op_short(op2, val1)) {
                        self.store_op_short(op1, val1);
                        self.store_op_short(op2, val2);
                    }
                }
            },
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
            },
            MOVES => {
                if let Some(val) = self.get_op_short(op1) {
                    if self.flag_get(ZERO_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVNES => {
                if let Some(val) = self.get_op_short(op1) {
                    if !self.flag_get(ZERO_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVLS => {
                if let Some(val) = self.get_op_short(op1) {
                    if self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVLES => {
                if let Some(val) = self.get_op_short(op1) {
                    if (self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            || self.flag_get(ZERO_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVGS => {
                if let Some(val) = self.get_op_short(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG))
                            && !self.flag_get(ZERO_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVGES => {
                if let Some(val) = self.get_op_short(op1) {
                    if !(self.flag_get(NEGATIVE_FLAG) ^ self.flag_get(OVERFLOW_FLAG)) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVLUS => {
                if let Some(val) = self.get_op_short(op1) {
                    if self.flag_get(CARRY_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVLEUS => {
                if let Some(val) = self.get_op_short(op1) {
                    if self.flag_get(CARRY_FLAG) || self.flag_get(ZERO_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVGUS => {
                if let Some(val) = self.get_op_short(op1) {
                    if !self.flag_get(CARRY_FLAG) && !self.flag_get(ZERO_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            },
            MOVGEUS => {
                if let Some(val) = self.get_op_short(op1) {
                    if !self.flag_get(CARRY_FLAG) {
                        self.store_op_short(op2, val);
                    }
                }
            }
        }
    }

    fn get_rp_after_jmp(&mut self, op: Operand) -> Option<u32> {
        if let Operand::Constant(c, t) = op {
            Some(match t {
                OffsetType::PositiveRelative => self.rp.wrapping_add(c),
                OffsetType::NegativeRelative => self.rp.wrapping_sub(c),
                OffsetType::AbsoluteNone => c
            })
        } else {
            self.get_op_long(op)
        }
    }
}
