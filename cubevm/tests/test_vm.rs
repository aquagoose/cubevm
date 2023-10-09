use cubevm::Instruction::*;
use cubevm::{StackValue, VmEngine};

#[test]
fn basic_math_test() {
    let instructions = [
        PushN(2.0),
        PushN(-1.0),
        Add
    ];

    let mut vm = VmEngine::new();
    vm.execute(&instructions);

    println!("{:?}", vm.stack_top());
    assert_eq!(vm.stack_top(), &StackValue::Number(1.0));

    let instructions = [
        PushN(5.0),
        Sub
    ];

    vm.execute(&instructions);

    println!("{:?}", vm.stack_top());
    assert_eq!(vm.stack_top(), &StackValue::Number(-4.0));

    let instructions = [
        PushN(4.0),
        Mul
    ];

    vm.execute(&instructions);

    println!("{:?}", vm.stack_top());
    assert_eq!(vm.stack_top(), &StackValue::Number(-16.0));

    let instructions = [
        PushN(5.0),
        Div
    ];

    vm.execute(&instructions);

    println!("{:?}", vm.stack_top());
    assert_eq!(vm.stack_top(), &StackValue::Number(-16.0 / 5.0));
}

#[test]
fn test_concat() {
    let mut vm = VmEngine::new();

    let instructions = [
        PushS("Hello".to_string()),
        PushS(" ".to_string()),
        PushS("World!".to_string()),
        Cncat(3)
    ];

    vm.execute(&instructions);

    println!("{:?}", vm.stack_top());
    assert_eq!(vm.stack_top(), &StackValue::String("Hello World!".to_string()));
}

#[test]
fn test_basic_function() {
    let mut vm = VmEngine::new();

    vm.register_function("test", |_| {
        println!("This is a test.");
        None
    });

    let instructions = [
        SCall("test".to_string())
    ];

    vm.execute(&instructions);
}

#[test]
fn test_advanced_function() {
    let mut vm = VmEngine::new();

    vm.register_function("addnum", |stack| {
        let rhs = match stack.pop_back().unwrap() {
            StackValue::Number(num) => num,
            _ => panic!("Number expected.")
        };

        let lhs = match stack.pop_back().unwrap() {
            StackValue::Number(num) => num,
            _ => panic!("Number expected.")
        };

        Some(StackValue::Number(lhs + rhs))
    });

    let instructions = [
        PushN(1.0),
        PushN(1.0),
        SCall("addnum".to_string())
    ];

    vm.execute(&instructions);

    println!("{:?}", vm.stack_top());
    assert_eq!(vm.stack_top(), &StackValue::Number(2.0));
}