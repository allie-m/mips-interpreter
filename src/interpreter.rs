use std::io::stdin;

#[derive(Debug)]
pub struct InterpreterState {
    pub program_counter: u32,
    pub register_file: [u32; 32],
    pub instruction_mem: Vec<u32>,
    pub data_mem: Vec<u8>,

    pub instruction_mem_start: u32,
    pub data_mem_start: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum InterpreterError {
    #[allow(dead_code)]
    // this isn't dead code silly if an instruction's invalid it'll be printed when the code panics
    InvalidInstruction(crate::instruction::DecodeErr),
    BadProgramCounter,
}

impl InterpreterState {
    pub fn process_next_instruction(&mut self) -> Result<bool, InterpreterError> {
        use crate::instruction::Instruction;
        let i_word = *self
            .instruction_mem
            .get(self.program_counter as usize / 4)
            .ok_or(InterpreterError::BadProgramCounter)?;
        let instruction = crate::instruction::decode(i_word)
            .map_err(|err| InterpreterError::InvalidInstruction(err))?;
        // println!("{:?}", instruction);
        match instruction {
            Instruction::Regular {
                kind,
                rs,
                rt,
                rd,
                shamt,
            } => {
                use crate::instruction::RegularInstructionType::*;
                let rs = rs as usize;
                let rt = rt as usize;
                let rd = rd as usize;
                match kind {
                    Add => {
                        self.register_file[rd] = u32::from_ne_bytes(
                            (i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                                + i32::from_ne_bytes(self.register_file[rt].to_ne_bytes()))
                            .to_ne_bytes(),
                        )
                    }
                    Addu => {
                        self.register_file[rd] = self.register_file[rs] + self.register_file[rt]
                    }
                    And => self.register_file[rd] = self.register_file[rs] & self.register_file[rt],
                    Jr => {
                        self.program_counter = self.register_file[rs];
                        return Ok(true);
                    }
                    Or => self.register_file[rd] = self.register_file[rs] | self.register_file[rt],
                    Nor => {
                        self.register_file[rd] = !(self.register_file[rs] | self.register_file[rt])
                    }
                    Slt => {
                        self.register_file[rd] =
                            (i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                                < i32::from_ne_bytes(self.register_file[rt].to_ne_bytes()))
                                as u32
                    }
                    Sltu => {
                        self.register_file[rd] =
                            (self.register_file[rs] < self.register_file[rt]) as u32
                    }
                    Sll => self.register_file[rd] = self.register_file[rt] << shamt,
                    Sra => {
                        self.register_file[rd] = u32::from_ne_bytes(
                            (i32::from_ne_bytes(self.register_file[rt].to_ne_bytes()) >> shamt)
                                .to_ne_bytes(),
                        )
                    }
                    Srl => self.register_file[rd] = self.register_file[rt] >> shamt,
                    Sub => {
                        let rs = i32::from_ne_bytes(self.register_file[rs].to_ne_bytes());
                        let rt = i32::from_ne_bytes(self.register_file[rt].to_ne_bytes());
                        self.register_file[rd] = u32::from_ne_bytes((rs - rt).to_ne_bytes());
                    }
                    Subu => {
                        self.register_file[rd] = self.register_file[rs] - self.register_file[rt]
                    }
                }
            }
            Instruction::Immediate {
                kind,
                rs,
                rt,
                immediate,
            } => {
                use crate::instruction::ImmediateInstructionType::*;
                let rs = rs as usize;
                let rt = rt as usize;
                let imm_zero_ext = u16::from_ne_bytes(immediate.to_ne_bytes()) as u32;
                match kind {
                    Addi => {
                        self.register_file[rt] = u32::from_ne_bytes(
                            (i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                                + immediate as i32)
                                .to_ne_bytes(),
                        )
                    }
                    Addiu => self.register_file[rt] = self.register_file[rs] + imm_zero_ext,
                    Andi => self.register_file[rt] = self.register_file[rs] & imm_zero_ext,
                    Beq => {
                        if self.register_file[rs] == self.register_file[rt] {
                            self.program_counter = self
                                .program_counter
                                .checked_add_signed((immediate as i32) * 4)
                                .unwrap();
                            return Ok(true);
                        }
                    }
                    Bne => {
                        if self.register_file[rs] != self.register_file[rt] {
                            self.program_counter = self
                                .program_counter
                                .checked_add_signed((immediate as i32) * 4)
                                .unwrap();
                            return Ok(true);
                        }
                    }
                    Lb => {
                        self.register_file[rt] =
                            self.data_mem[(self.register_file[rs] as i32 + immediate as i32)
                                as usize
                                - self.data_mem_start] as u32;
                    }
                    Lbu => todo!(),
                    Lhu => {
                        let addr = (self.register_file[rs] as i32 + immediate as i32) as usize
                            - self.data_mem_start;
                        self.register_file[rt] = self.data_mem[addr] as u32;
                        self.register_file[rt] |= (self.data_mem[addr + 1] as u32) << 8;
                    }
                    Ll => todo!(),
                    Lui => {
                        self.register_file[rt] =
                            (u32::from_ne_bytes((immediate as i32).to_ne_bytes())) << 16
                    }
                    Lw => {
                        let addr = (self.register_file[rs] as i32 + immediate as i32) as usize
                            - self.data_mem_start;
                        self.register_file[rt] = self.data_mem[addr] as u32;
                        self.register_file[rt] |= (self.data_mem[addr + 1] as u32) << 8;
                        self.register_file[rt] |= (self.data_mem[addr + 2] as u32) << 16;
                        self.register_file[rt] |= (self.data_mem[addr + 3] as u32) << 24;
                    }
                    Ori => self.register_file[rt] = self.register_file[rs] | imm_zero_ext,
                    Slti => {
                        self.register_file[rt] =
                            (i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                                < immediate as i32) as u32
                    }
                    Sltiu => {
                        self.register_file[rt] = (self.register_file[rs] < imm_zero_ext) as u32
                    }
                    Sb => {
                        self.data_mem[(i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                            + immediate as i32) as usize
                            - self.data_mem_start] = self.register_file[rt] as u8
                    }
                    Sc => todo!(),
                    Sh => {
                        let addr = (i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                            + immediate as i32) as usize
                            - self.data_mem_start;
                        self.data_mem[addr] = self.register_file[rt] as u8;
                        self.data_mem[addr + 1] = (self.register_file[rt] >> 8) as u8;
                    }
                    Sw => {
                        let addr = (i32::from_ne_bytes(self.register_file[rs].to_ne_bytes())
                            + immediate as i32) as usize
                            - self.data_mem_start;
                        self.data_mem[addr] = self.register_file[rt] as u8;
                        self.data_mem[addr + 1] = (self.register_file[rt] >> 8) as u8;
                        self.data_mem[addr + 2] = (self.register_file[rt] >> 16) as u8;
                        self.data_mem[addr + 3] = (self.register_file[rt] >> 24) as u8;
                    }
                }
            }
            Instruction::Jump { kind, address } => {
                use crate::instruction::JumpInstructionType::*;
                let address = ((self.program_counter + 4) & 0xf0000000) | (address << 2);
                match kind {
                    Jump => {
                        self.program_counter = address - self.instruction_mem_start;
                    }
                    JumpAndLink => {
                        self.register_file[31] = self.program_counter + 4;
                        self.program_counter = address - self.instruction_mem_start;
                    }
                }
                // no incrementing the program counter
                // we've alreadyyy done that
                return Ok(true);
            }
            Instruction::Syscall => {
                // https://peterfab.com/ref/mips/syscalls.html
                let v0 = self.register_file[2];
                let a0 = self.register_file[4];
                let a1 = self.register_file[5];
                match v0 {
                    // print int
                    1 => print!("{}", i32::from_ne_bytes(a0.to_ne_bytes())),
                    // print float
                    2 => panic!("floats not supported yet!"),
                    // print double
                    3 => panic!("floats not supported yet!"),
                    // print string
                    4 => {
                        let mut i = a0 as usize - self.data_mem_start;
                        loop {
                            if self.data_mem[i] == 0 { break; }
                            print!("{}", self.data_mem[i] as char);
                            i += 1;
                        }
                    }
                    // read int
                    5 => {
                        self.register_file[2] = stdin()
                            .lines()
                            .find_map(|a| Some(a.ok()?.parse().ok()?))
                            .unwrap();
                    }
                    // read float
                    6 => panic!("floats not supported yet!"),
                    // read double
                    7 => panic!("floats not supported yet!"),
                    // read string
                    8 => {
                        let string = stdin().lines().next().unwrap().unwrap();
                        let mut i = a0 as usize - self.data_mem_start;
                        let mut j = 0;
                        for b in string.bytes() {
                            if j >= a1 - 2 {
                                break;
                            }
                            self.data_mem[i] = b;
                            i += 1;
                            j += 1;
                        }
                        // add a newline and null terminator
                        self.data_mem[i] = b'\n';
                        self.data_mem[i+1] = 0;
                    }
                    // memory allocation
                    9 => todo!(),
                    // exit
                    10 => return Ok(false),
                    // print character
                    11 => print!("{}", a0 as u8 as char),
                    // read character
                    12 => {
                        self.register_file[2] = stdin()
                            .lines()
                            .next()
                            .unwrap()
                            .unwrap()
                            .chars()
                            .next()
                            .unwrap() as u32;
                    }
                    other => panic!("unimplemented syscall {}", other),
                }
            }
        }
        self.program_counter += 4;
        Ok(true)
    }
}
