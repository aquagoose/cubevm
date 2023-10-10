use std::collections::VecDeque;
use std::io::Write;
use rand::Rng;
use cubevm::{StackValue, VmEngine};
use cubevm::Instruction::*;

fn log(stack: &mut VecDeque<StackValue>) -> Option<StackValue> {
    match stack.pop_back().unwrap() {
        StackValue::Number(num) => println!("{}", num),
        StackValue::String(string) => println!("{}", string),
        StackValue::Bool(b) => println!("{}", b)
    }

    None
}

fn prompt(stack: &mut VecDeque<StackValue>) -> Option<StackValue> {
    match stack.pop_back().unwrap() {
        StackValue::Number(num) => print!("{}", num),
        StackValue::String(string) => print!("{}", string),
        StackValue::Bool(b) => print!("{}", b)
    }

    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.ends_with("\n") {
        input = input[..input.len() - 2].to_string();
    }

    Some(StackValue::String(input))
}

fn rand(stack: &mut VecDeque<StackValue>) -> Option<StackValue> {
    let max = match stack.pop_back().unwrap() {
        StackValue::Number(num) => num,
        _ => panic!("Number expected.")
    } as i32;

    let min = match stack.pop_back().unwrap() {
        StackValue::Number(num) => num,
        _ => panic!("Number expected.")
    } as i32;

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(min..=max);

    Some(StackValue::Number(num as f64))
}

#[test]
fn test_name_age() {
    let mut vm = VmEngine::new();
    vm.register_function("log", log);
    vm.register_function("prompt", prompt);

    /*
        cubelang equivalent code:

        name is prompt("Hello! What is your name? ")

        loop
            age is num(prompt("Hi {name}, how old are you? "))

            if age > 100
                log("Wow, you are old. Get younger.")
                terminate
            else if age < 0
                log("Lol. Sure. That ain't right.")
            else
                break
            end
        end

        log("That's cool, {name}, you're {age} years old!")
     */

    let instructions = [
        PushS("Hello! What is your name? ".to_string()),
        SCall("prompt".to_string()),
        StReg(0),

        PushS("Hi ".to_string()),
        LdReg(0),
        PushS(", how old are you? ".to_string()),
        Cncat(3),
        SCall("prompt".to_string()),
        ToNum,
        StReg(1),

        LdReg(1),
        PushN(100.0),
        Brlst(16),
        PushS("Wow, you are old. Get younger.".to_string()),
        SCall("log".to_string()),
        TermP,

        LdReg(1),
        PushN(0.0),
        Brgrt(22),
        PushS("Lol. Sure. That ain't right.".to_string()),
        SCall("log".to_string()),
        Brnch(3),

        PushS("That's cool, ".to_string()),
        LdReg(0),
        PushS(", you're ".to_string()),
        LdReg(1),
        PushS(" years old!".to_string()),
        Cncat(5),
        SCall("log".to_string())
    ];

    vm.execute(&instructions);
}

#[test]
fn number_game() {
    let mut vm = VmEngine::new();
    vm.register_function("log", log);
    vm.register_function("prompt", prompt);
    vm.register_function("rand", rand);

    let instructions = [
        PushN(50.0),
        StReg(0),
        PushN(1.0),
        PushN(50.0),
        SCall("rand".to_string()),
        SCall("log".to_string()),
        LdReg(0),
        PushN(1.0),
        Sub,
        PushN(0.0),
        Brgrt(2)
    ];

    vm.execute(&instructions);
}