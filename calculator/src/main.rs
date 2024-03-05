use std::io;

struct MathematicalOperations {
    number1: i128,
    mathematical_operator: char,
    number2: i128,
}

fn main() {
    println!("---RUST CALCULATOR---");

    println!("Enter the expression (e.g., 2 + 2):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    let number1: i128 = parts[0].parse().expect("Invalid number");
    let mathematical_operator = parts[1].chars().next().expect("Invalid operator");
    let number2: i128 = parts[2].parse().expect("Invalid number");

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
    x.pow(y as u32)
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
