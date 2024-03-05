use std::io;

struct MathematicalOperations {
    number1: i128,
    mathematical_operator: char,
    number2: i128,
}

fn main() {
    println!("---RUST CALCULATOR---");

    println!("Enter the first number:");
    let mut number1 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Error");

    let number1: i128 = number1.trim().parse().expect("Error");

    println!("Enter the mathematical operator:");
    let mut mathematical_operator = String::new();
    io::stdin()
        .read_line(&mut mathematical_operator)
        .expect("Error");

    let mathematical_operator = mathematical_operator.trim().chars().next().expect("Error");

    println!("Enter the second number:");
    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Error");

    let number2: i128 = number2.trim().parse().expect("Error");

    let operation = MathematicalOperations {
        number1,
        mathematical_operator,
        number2,
    };

    let result = match operation.mathematical_operator {
        '+' => addition(operation.number1, operation.number2),
        '-' => subtraction(operation.number1, operation.number2),
        '*' => multiplication(operation.number1, operation.number2),
        '/' => division(operation.number1, operation.number2),
        '^' => exponentiation(operation.number1, operation.number2),
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}

fn addition(x: i128, y: i128) -> i128 {
    x + y
}

fn exponentiation(x: i128, y: i128) -> i128 {
    let mut result = 1;
    for _ in 0..y {
        result *= x;
    }
    result
}

fn subtraction(x: i128, y: i128) -> i128 {
    x - y
}

fn multiplication(x: i128, y: i128) -> i128 {
    x * y
}

fn division(x: i128, y: i128) -> i128 {
    if y == 0 {
        panic!("Division by zero");
    }
    x / y
}
