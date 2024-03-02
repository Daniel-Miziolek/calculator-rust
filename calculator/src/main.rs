use std::io;

fn main() {
    loop {
        println!("---Calculator---");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplicatione");
        println!("4. Division");
        println!("5. Exponentiation");
        println!("6. Root");
        println!("7. Logarithm");
        println!("8. Absolute value");
        println!("0. Exit");

        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Error");

        let choose: i8 = match text.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Error, try again.");
                continue;
            }
        };

        match choose {
            0 => {
                println!("End.");
                break;
            }
            1 => addition(),
            2 => subtraction(),
            3 => multiplicatione(),
            4 => division(),
            5 => exponentiation(),
            6 => root(),
            7 => logarithm(),
            8 => ab(),
            _ => println!("Error, try again."),
        }
    }
}





fn addition() {
    println!("Set a first number: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error");
    let n1:i8 = text.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let n2:i8 = text2.trim().parse()
        .expect("Error");

    let result = n1+n2;

    println!("Result: {}",result);
}

fn subtraction() {
    println!("Set a first number: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error");
    let n1:i8 = text.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let n2:i8 = text2.trim().parse()
        .expect("Error");

    let result = n1-n2;

    println!("Result: {}",result);
}

fn multiplicatione() {
    println!("Set a first number: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error");
    let n1:i8 = text.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let n2:i8 = text2.trim().parse()
        .expect("Error");

    let result = n1*n2;

    println!("Result: {}",result);
}

fn division(){
    println!("Set a first number: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error");
    let n1:i8 = text.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let n2:i8 = text2.trim().parse()
        .expect("Error");

    let result = n1/n2;

    println!("Result: {}",result);
}

fn exponentiation(){
    println!("Give the base of the power: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error");
    let n1:i8 = text.trim().parse()
        .expect("Error");

    println!("Give the exponent of the power: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let n2:i8 = text2.trim().parse()
        .expect("Error");

    let result = n1.pow(n2.try_into().unwrap());

    println!("Result: {}",result);
}

fn root(){
    println!("Enter the degree of the element:");
    let mut degree = String::new();
    io::stdin().read_line(&mut degree).expect("Error ");
    let degree: f64 = degree.trim().parse().expect("Error");

    
    println!("Enter the number under the square root:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error");
    let number: f64 = number.trim().parse().expect("Error");

    
    let result = number.powf(1.0 / degree);

   
    println!("Result: {}",result);
}

fn logarithm(){
    println!("Enter the base of the logarithm: ");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Error ");
    let base: f64 = base.trim().parse().expect("Error");


    println!("Enter the logarithmic number: ");
    let mut log = String::new();
    io::stdin().read_line(&mut log).expect("Error");
    let log: f64 = log.trim().parse().expect("Error");


    let result = log.ln() / base.ln();


    println!("Result: {}",result);
}

fn ab(){
    println!("Enter the number: ");
    let mut ab:String = String::new();
    io::stdin()
        .read_line(&mut ab)
        .expect("Error");

    let ab:f64 = ab.trim().parse().expect("Error");

    let result:f64 = ab.abs();

    println!("Result: {}", result);
}



