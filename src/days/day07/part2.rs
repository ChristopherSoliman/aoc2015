use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Value<'a> {
    Register(&'a str),
    Constant(u16),
}

impl<'a> Value<'a> {
    pub fn from_str(a: &str) -> Value {
        if a.chars().next().unwrap().is_alphabetic() {
            Value::Register(a.trim())
        } else {
            Value::Constant(a.parse().unwrap())
        }
    }

    pub fn get_value(&self, regs: &HashMap<&str, u16>) -> Option<u16> {
        match self {
            Value::Constant(v) => Some(*v),
            Value::Register(reg) => {
                if let Some(v) = regs.get(*reg) {
                    Some(*v)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
enum Operation<'a> {
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    LShift(Value<'a>, Value<'a>),
    RShift(Value<'a>, Value<'a>),
    Not(Value<'a>),
    Equal(Value<'a>),
}

impl<'a> Operation<'a> {
    pub fn compute(&self, regs: &HashMap<&str, u16>) -> Option<u16> {
        match self {
            Operation::And(a, b) => {
                let a = a.get_value(regs);
                let b = b.get_value(regs);
                if a == None || b == None {
                    return None;
                }
                Some(a.unwrap() & b.unwrap())
            }
            Operation::Or(a, b) => {
                let a = a.get_value(regs);
                let b = b.get_value(regs);
                if a == None || b == None {
                    return None;
                }
                Some(a.unwrap() | b.unwrap())
            }
            Operation::LShift(a, b) => {
                let a = a.get_value(regs);
                let b = b.get_value(regs);
                if a == None || b == None {
                    return None;
                }
                Some(a.unwrap() << b.unwrap())
            }
            Operation::RShift(a, b) => {
                let a = a.get_value(regs);
                let b = b.get_value(regs);
                if a == None || b == None {
                    return None;
                }
                Some(a.unwrap() >> b.unwrap())
            }
            Operation::Not(a) => {
                let a = a.get_value(regs);
                if a == None {
                    return None;
                }
                Some(!a.unwrap())
            }
            Operation::Equal(a) => a.get_value(regs),
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    op: Operation<'a>,
    out: &'a str,
}

pub fn run(path: &str) -> u16 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut regs: HashMap<&str, u16> = HashMap::new();
    let mut inst: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        let (ins, out) = line.split_once("->").unwrap();
        let ins = ins.split_whitespace().collect::<Vec<_>>();
        if ins.len() == 1 {
            if out.trim() == "b" {
                continue;
            }
            inst.push(Instruction {
                op: Operation::Equal(Value::from_str(ins[0])),
                out: out.trim(),
            });
        } else if ins.len() == 2 {
            inst.push(Instruction {
                op: Operation::Not(Value::from_str(ins[1])),
                out: out.trim(),
            });
        } else {
            match ins[1] {
                "AND" => inst.push(Instruction {
                    op: Operation::And(Value::from_str(ins[0]), Value::from_str(ins[2])),
                    out: out.trim(),
                }),
                "OR" => inst.push(Instruction {
                    op: Operation::Or(Value::from_str(ins[0]), Value::from_str(ins[2])),
                    out: out.trim(),
                }),

                "LSHIFT" => inst.push(Instruction {
                    op: Operation::LShift(Value::from_str(ins[0]), Value::from_str(ins[2])),
                    out: out.trim(),
                }),
                "RSHIFT" => inst.push(Instruction {
                    op: Operation::RShift(Value::from_str(ins[0]), Value::from_str(ins[2])),
                    out: out.trim(),
                }),
                _ => unreachable!("invalid operation"),
            }
        }
    }

    let mut all_done: bool;
    regs.insert("b", 956);

    loop {
        all_done = true;
        for instruction in &inst {
            if let Some(val) = instruction.op.compute(&regs) {
                regs.insert(instruction.out, val);
            } else {
                all_done = false;
            }
        }
        if all_done {
            break;
        }
    }

    if let Some(v) = regs.get(&"a") {
        return *v;
    }
    panic!("a register not set");
}
