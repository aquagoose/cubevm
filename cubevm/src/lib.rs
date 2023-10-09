use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
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

    SCall(String),

    FCall(String),

    Brnch(String),

    Brzer(String),

    Brone(String),

    Brequ(String),

    Retrn,

    Cncat(usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    Number(f64),
    String(String),
    Bool(bool)
}

pub struct VmEngine {
    stack: VecDeque<StackValue>,
    registers: [Option<StackValue>; 9],
    functions: HashMap<String, fn(&mut VecDeque<StackValue>) -> Option<StackValue>>
}

impl VmEngine {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            registers: std::array::from_fn(|_| None),
            functions: HashMap::new()
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
                Instruction::LdReg(id) => self.stack.push_back(self.registers[*id as usize].clone().unwrap()),
                Instruction::LdVar(_) => todo!(),
                Instruction::Pop => {
                    self.stack.pop_back();
                },

                // These instructions pop the rhs first.
                // That's because the math order in cubevm is in reverse to the stack order.
                // For example, if "3" is pushed, and then "4", it will be treated as 3 + 4, even
                // though technically it's backwards to the stack order.
                Instruction::Add => {
                    let (lhs, rhs) = self.pop_two_numbers();
                    self.stack.push_back(StackValue::Number(lhs + rhs))
                },

                Instruction::Sub => {
                    let (lhs, rhs) = self.pop_two_numbers();
                    self.stack.push_back(StackValue::Number(lhs - rhs))
                },

                Instruction::Mul => {
                    let (lhs, rhs) = self.pop_two_numbers();
                    self.stack.push_back(StackValue::Number(lhs * rhs))
                },

                Instruction::Div => {
                    let (lhs, rhs) = self.pop_two_numbers();
                    self.stack.push_back(StackValue::Number(lhs / rhs))
                },

                Instruction::SCall(name) => {
                    if let Some(value) = self.functions.get(name).unwrap()(&mut self.stack) {
                        self.stack.push_back(value);
                    }
                },

                Instruction::FCall(name) => todo!(),

                Instruction::Brnch(_) => todo!(),
                Instruction::Brzer(_) => todo!(),
                Instruction::Brone(_) => todo!(),
                Instruction::Brequ(_) => todo!(),
                Instruction::Retrn => todo!(),
                Instruction::Cncat(num) => {
                    let mut text = String::new();

                    for _ in 0..*num {
                        text.insert_str(0, &match self.stack.pop_back().unwrap() {
                            StackValue::String(string) => string.clone(),
                            StackValue::Number(num) => num.to_string(),
                            StackValue::Bool(b) => b.to_string()
                        });
                    }

                    self.stack.push_back(StackValue::String(text));
                }
            }
        }
    }

    pub fn stack_top(&self) -> &StackValue {
        &self.stack.back().unwrap()
    }

    pub fn register_function(&mut self, name: &str, function: fn(&mut VecDeque<StackValue>) -> Option<StackValue>) {
        self.functions.insert(name.to_string(), function);
    }

    fn pop_two_numbers(&mut self) -> (f64, f64) {
        let rhs = match self.stack.pop_back().unwrap() {
            StackValue::Number(num) => num,
            _ => panic!("Value must be number.")
        };

        let lhs = match self.stack.pop_back().unwrap() {
            StackValue::Number(num) => num,
            _ => panic!("Value must be number.")
        };

        (lhs, rhs)
    }
}