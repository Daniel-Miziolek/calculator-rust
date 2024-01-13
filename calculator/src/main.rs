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
            1 => dodawanie(),
            2 => odejmowanie(),
            3 => mnozenie(),
            4 => dzielenie(),
            5 => potegowanie(),
            6 => pierwiastkowanie(),
            _ => println!("Error, try again."),
        }
    }
}





fn dodawanie() {
    println!("Set a first number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let choose2:i8 = text2.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Error");
    let choose3:i8 = text3.trim().parse()
        .expect("Error");

    let mut wynik = choose2+choose3;

    println!("Result: {}",wynik);
}

fn odejmowanie() {
    println!("Set a first number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let choose2:i8 = text2.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Error");
    let choose3:i8 = text3.trim().parse()
        .expect("Error");

    let mut wynik = choose2-choose3;

    println!("Result: {}",wynik);
}

fn mnozenie() {
    println!("Set a first number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let choose2:i8 = text2.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Error");
    let choose3:i8 = text3.trim().parse()
        .expect("Error");

    let mut wynik = choose2*choose3;

    println!("Result: {}",wynik);
}

fn dzielenie(){
    println!("Set a first number: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let choose2:i8 = text2.trim().parse()
        .expect("Error");

    println!("Set a second number: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Error");
    let choose3:i8 = text3.trim().parse()
        .expect("Error");

    let mut wynik = choose2/choose3;

    println!("Result: {}",wynik);
}

fn potegowanie(){
    println!("Give the base of the power: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Error");
    let choose2:i8 = text2.trim().parse()
        .expect("Error");

    println!("Give the exponent of the power: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Error");
    let choose3:i8 = text3.trim().parse()
        .expect("Error");

    let mut wynik = choose2.pow(choose3.try_into().unwrap());

    println!("Result: {}",wynik);
}

fn pierwiastkowanie(){
    println!("Enter the degree of the element:");
    let mut degree = String::new();
    io::stdin().read_line(&mut degree).expect("Error ");
    let degree: f64 = degree.trim().parse().expect("Error");

    // Pobranie liczby od użytkownika
    println!("Enter the number under the square root:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error");
    let number: f64 = number.trim().parse().expect("Error");

    // Obliczenie pierwiastka
    let result = number.powf(1.0 / degree);

    // Wyświetlenie wyniku
    println!("Result: {}",result);
}
