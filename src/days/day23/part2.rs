enum Register {
    A,
    B,
}
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

struct Computer {
    reg_a: usize,
    reg_b: usize,
    ipointer: usize,
    instructions: Vec<Instruction>,
}

impl Computer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Computer {
            reg_a: 1,
            reg_b: 0,
            ipointer: 0,
            instructions,
        }
    }

    pub fn run(&mut self) {
        loop {
            match &self.instructions[self.ipointer] {
                Instruction::Half(register) => {
                    let reg = match register {
                        Register::A => &mut self.reg_a,
                        Register::B => &mut self.reg_b,
                    };
                    *reg /= 2;
                }
                Instruction::Triple(register) => {
                    let reg = match register {
                        Register::A => &mut self.reg_a,
                        Register::B => &mut self.reg_b,
                    };
                    *reg *= 3;
                }
                Instruction::Increment(register) => {
                    let reg = match register {
                        Register::A => &mut self.reg_a,
                        Register::B => &mut self.reg_b,
                    };
                    *reg += 1;
                }
                Instruction::Jump(offset) => {
                    let np = self.ipointer as i32 + offset;
                    if np < 0 || np >= self.instructions.len() as i32 {
                        break;
                    }
                    self.ipointer = np as usize;
                    continue;
                }
                Instruction::JumpIfEven(register, offset) => {
                    let reg = match register {
                        Register::A => &mut self.reg_a,
                        Register::B => &mut self.reg_b,
                    };
                    if *reg % 2 == 0 {
                        let np = self.ipointer as i32 + offset;
                        if np < 0 || np >= self.instructions.len() as i32 {
                            break;
                        }
                        self.ipointer = np as usize;
                        continue;
                    }
                }
                Instruction::JumpIfOne(register, offset) => {
                    let reg = match register {
                        Register::A => &mut self.reg_a,
                        Register::B => &mut self.reg_b,
                    };
                    if *reg == 1 {
                        let np = self.ipointer as i32 + offset;
                        if np < 0 || np >= self.instructions.len() as i32 {
                            break;
                        }
                        self.ipointer = np as usize;
                        continue;
                    }
                }
            }
            self.ipointer += 1;
        }
    }

    fn get_register(&self, reg: &Register) -> &usize {
        match reg {
            Register::A => &self.reg_a,
            Register::B => &self.reg_b,
        }
    }
}

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let instructions = input
        .lines()
        .map(|line| {
            let (inst, op) = line.split_once(" ").unwrap();
            match inst {
                "jio" => {
                    let (reg, off) = op.split_once(", ").unwrap();
                    let off = off.parse::<i32>().unwrap();
                    let reg = if reg == "a" { Register::A } else { Register::B };
                    Instruction::JumpIfOne(reg, off)
                }
                "jie" => {
                    let (reg, off) = op.split_once(", ").unwrap();
                    let off = off.parse::<i32>().unwrap();
                    let reg = if reg == "a" { Register::A } else { Register::B };
                    Instruction::JumpIfEven(reg, off)
                }
                "jmp" => {
                    let off = op.parse::<i32>().unwrap();
                    Instruction::Jump(off)
                }
                "hlf" => {
                    let reg = if op == "a" { Register::A } else { Register::B };
                    Instruction::Half(reg)
                }
                "tpl" => {
                    let reg = if op == "a" { Register::A } else { Register::B };
                    Instruction::Triple(reg)
                }
                "inc" => {
                    let reg = if op == "a" { Register::A } else { Register::B };
                    Instruction::Increment(reg)
                }
                _ => unreachable!("invalid operation"),
            }
        })
        .collect::<Vec<_>>();

    let mut computer = Computer::new(instructions);
    computer.run();
    *computer.get_register(&Register::B)
}
