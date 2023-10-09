use std::collections::VecDeque;

pub enum Instruction {
    PushN(f64),

    PushS(String),

    PushB(bool),

    StReg(u8),

    StVar(String),

    LdReg(u8),

    LdVar(String),

    Pop,

    Add,

    Sub,

    Mul,

    Div,

    Brnch(String),

    Brzer(String),

    Brone(String),

    Brequ(String),

    Retrn,

    Cncat(usize)
}

pub enum StackValue {
    Number(f64),
    String(String),
    Bool(bool)
}

pub struct VmEngine {
    stack: VecDeque<StackValue>,
    registers: [Option<StackValue>; 9]
}

impl VmEngine {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            registers: [None; 9]
        }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::PushN(num) => self.stack.push_back(StackValue::Number(*num)),
                Instruction::PushS(string) => self.stack.push_back(StackValue::String(string.clone())),
                Instruction::PushB(b) => self.stack.push_back(StackValue::Bool(*b)),
                Instruction::StReg(id) => self.registers[*id as usize] = Some(self.stack.pop_back().unwrap()),
                Instruction::StVar(_) => todo!(),
                Instruction::LdReg(id) => self.stack.push_back(self.registers[*id as usize].unwrap()),
                Instruction::LdVar(_) => todo!(),
                Instruction::Pop => self.stack.pop_back(),

                // These instructions pop the rhs first.
                // That's because the math order in cubevm is in reverse to the stack order.
                // For example, if "3" is pushed, and then "4", it will be treated as 3 + 4, even
                // though technically it's backwards to the stack order.
                Instruction::Add => {
                    let rhs = match self.stack.pop_back().unwrap() {
                        StackValue::Number(num) => num,
                        _ => panic!("Value must be number.")
                    };

                    let lhs = match self.stack.pop_back().unwrap() {
                        StackValue::Number(num) => num,
                        _ => panic!("Value must be number.")
                    };

                    self.stack.push_back()
                }
                Instruction::Sub => {}
                Instruction::Mul => {}
                Instruction::Div => {}

                Instruction::Brnch(_) => {}
                Instruction::Brzer(_) => {}
                Instruction::Brone(_) => {}
                Instruction::Brequ(_) => {}
                Instruction::Retrn => {}
                Instruction::Cncat(_) => {}
            }
        }
    }
}