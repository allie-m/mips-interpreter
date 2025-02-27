use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Regular {
        kind: RegularInstructionType,
        rs: u32,    // 5 bits
        rt: u32,    // 5 bits
        rd: u32,    // 5 bits
        shamt: u32, // 5 bits
    },
    Immediate {
        kind: ImmediateInstructionType,
        rs: u32, // 5 bits
        rt: u32, // 5 bits
        immediate: i16,
    },
    Jump {
        kind: JumpInstructionType,
        address: u32, // 26 bits
    },
    Syscall,
}

#[derive(Clone, Copy, Debug)]
pub enum RegularInstructionType {
    Add,
    Addu,
    And,
    Jr,
    Nor,
    Or,
    Slt,
    Sltu,
    Sll,
    Sra,
    Srl,
    Sub,
    Subu,
}

#[derive(Clone, Copy, Debug)]
pub enum ImmediateInstructionType {
    Addi,
    Addiu,
    Andi,
    Beq,
    Bne,
    Lb,
    Lbu,
    Lhu,
    Ll,
    Lui,
    Lw,
    Ori,
    Slti,
    Sltiu,
    Sb,
    Sc,
    Sh,
    Sw,
}

#[derive(Clone, Copy, Debug)]
pub enum JumpInstructionType {
    Jump,
    JumpAndLink,
}

#[derive(Clone, Copy)]
pub enum DecodeErr {
    UnknownRegularInstruction { funct: u32 },
    UnknownImmediateInstruction { opcode: u32 },
}

impl Debug for DecodeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownRegularInstruction { funct } => {
                write!(f, "Unknown regular instruction {:#x}", funct)?
            }
            Self::UnknownImmediateInstruction { opcode } => {
                write!(f, "Unknown immediate instruction {:#x}", opcode)?
            }
        }
        Ok(())
    }
}

pub fn decode(word: u32) -> Result<Instruction, DecodeErr> {
    let opcode = word >> 26;
    match opcode {
        // jump instructions
        kind @ (0x2 | 0x3) => {
            use JumpInstructionType::*;
            let address = word & ((1 << 26) - 1);
            Ok(Instruction::Jump {
                kind: match kind {
                    0x2 => Jump,
                    0x3 => JumpAndLink,
                    _ => unreachable!(),
                },
                address,
            })
        }
        opcode => {
            let rs = (word >> 21) & 0b11111;
            let rt = (word >> 16) & 0b11111;
            match opcode {
                // regular instructions
                0 => {
                    use RegularInstructionType::*;
                    let rd = (word >> 11) & 0b11111;
                    let shamt = (word >> 6) & 0b11111;
                    let funct = word & 0b111_111;
                    if funct == 0xc {
                        return Ok(Instruction::Syscall);
                    }
                    Ok(Instruction::Regular {
                        kind: match funct {
                            0x20 => Add,
                            0x21 => Addu,
                            0x24 => And,
                            0x08 => Jr,
                            0x27 => Nor,
                            0x25 => Or,
                            0x2a => Slt,
                            0x2b => Sltu,
                            0x00 => Sll,
                            0x03 => Sra,
                            0x02 => Srl,
                            0x22 => Sub,
                            0x23 => Subu,
                            _ => Err(DecodeErr::UnknownRegularInstruction { funct })?,
                        },
                        rs,
                        rt,
                        rd,
                        shamt,
                    })
                }
                // immediate instructions
                opcode => {
                    use ImmediateInstructionType::*;
                    let immediate = (word & ((1 << 16) - 1)).to_le_bytes();
                    let immediate = i16::from_le_bytes([immediate[0], immediate[1]]);
                    Ok(Instruction::Immediate {
                        kind: match opcode {
                            0x08 => Addi,
                            0x09 => Addiu,
                            0x0c => Andi,
                            0x04 => Beq,
                            0x05 => Bne,
                            0x20 => Lb,
                            0x24 => Lbu,
                            0x25 => Lhu,
                            0x30 => Ll,
                            0x0f => Lui,
                            0x23 => Lw,
                            0x0d => Ori,
                            0x0a => Slti,
                            0x0b => Sltiu,
                            0x28 => Sb,
                            0x38 => Sc,
                            0x29 => Sh,
                            0x2b => Sw,
                            _ => Err(DecodeErr::UnknownImmediateInstruction { opcode })?,
                        },
                        rs,
                        rt,
                        immediate,
                    })
                }
            }
        }
    }
}
