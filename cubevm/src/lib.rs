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

    ToNum,

    SCall(String),

    FCall(String),

    Brnch(usize),

    Brzer(usize),

    Brone(usize),

    Brequ(usize),

    Brgrt(usize),

    Brlst(usize),

    Retrn,

    TermP,

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
        let mut i = 0;
        'main: while i < instructions.len() {
            let instruction = &instructions[i];

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

                Instruction::ToNum => {
                    let num = StackValue::Number(match self.stack.pop_back().unwrap() {
                        StackValue::Number(num) => num,
                        StackValue::String(string) => string.parse::<f64>().unwrap(),
                        StackValue::Bool(b) => b.into()
                    });

                    self.stack.push_back(num);
                }

                Instruction::SCall(name) => {
                    if let Some(value) = self.functions.get(name).unwrap()(&mut self.stack) {
                        self.stack.push_back(value);
                    }
                },

                Instruction::FCall(name) => todo!(),

                Instruction::Brnch(line) => {
                    i = *line;
                    continue;
                },

                Instruction::Brzer(line) => {
                    match self.stack.pop_back().unwrap() {
                        StackValue::Number(num) => if num == 0.0 { i = *line },
                        StackValue::Bool(b) => if !b { i = *line },
                        _ => panic!("Number or boolean expected.")
                    };

                    continue;
                },

                Instruction::Brone(line) => {
                    match self.stack.pop_back().unwrap() {
                        StackValue::Number(num) => if num == 1.0 { i = *line },
                        StackValue::Bool(b) => if b { i = *line },
                        _ => panic!("Number or boolean expected.")
                    };

                    continue;
                },

                Instruction::Brequ(line) => {
                    if self.stack.pop_back().unwrap() == self.stack.pop_back().unwrap() {
                        i = *line;
                        continue;
                    }
                },

                Instruction::Brgrt(line) => {
                    let (lhs, rhs) = self.pop_two_numbers();
                    if lhs > rhs {
                        i = *line;
                        continue;
                    }
                },

                Instruction::Brlst(line) => {
                    let (lhs, rhs) = self.pop_two_numbers();
                    if lhs < rhs {
                        i = *line;
                        continue;
                    }
                }

                Instruction::Retrn => todo!(),

                Instruction::TermP => break 'main,

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

            i += 1;
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