use std::io;

fn main() {
    loop {
        println!("---Kalkulator---");
        println!("1. Dodawanie");
        println!("2. Odejmowanie");
        println!("3. Mnożenie");
        println!("4. Dzielenie");
        println!("5. Potęgowanie");
        println!("6. Pierwiastkowanie");
        println!("0. Wyjście");

        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Błąd");

        let choose: i8 = match text.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Nieprawidłowe wejście. Spróbuj ponownie.");
                continue;
            }
        };

        match choose {
            0 => {
                println!("Do widzenia!");
                break;
            }
            1 => dodawanie(),
            2 => odejmowanie(),
            3 => mnozenie(),
            4 => dzielenie(),
            5 => potegowanie(),
            6 => pierwiastkowanie(),
            _ => println!("Nieprawidłowy wybór. Spróbuj ponownie."),
        }
    }
}

// ... (rest of your functions remain unchanged)



fn dodawanie() {
    println!("Podaj 1 liczbę: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Błąd");
    let choose2:i8 = text2.trim().parse()
        .expect("Błąd");

    println!("Podaj 2 liczbę: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Błąd");
    let choose3:i8 = text3.trim().parse()
        .expect("Błąd");

    let mut wynik = choose2+choose3;

    println!("Wynik: {}",wynik);
}

fn odejmowanie() {
    println!("Podaj 1 liczbę: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Błąd");
    let choose2:i8 = text2.trim().parse()
        .expect("Błąd");

    println!("Podaj 2 liczbę: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Błąd");
    let choose3:i8 = text3.trim().parse()
        .expect("Błąd");

    let mut wynik = choose2-choose3;

    println!("Wynik: {}",wynik);
}

fn mnozenie() {
    println!("Podaj 1 liczbę: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Błąd");
    let choose2:i8 = text2.trim().parse()
        .expect("Błąd");

    println!("Podaj 2 liczbę: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Błąd");
    let choose3:i8 = text3.trim().parse()
        .expect("Błąd");

    let mut wynik = choose2*choose3;

    println!("Wynik: {}",wynik);
}

fn dzielenie(){
    println!("Podaj 1 liczbę: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Błąd");
    let choose2:i8 = text2.trim().parse()
        .expect("Błąd");

    println!("Podaj 2 liczbę: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Błąd");
    let choose3:i8 = text3.trim().parse()
        .expect("Błąd");

    let mut wynik = choose2/choose3;

    println!("Wynik: {}",wynik);
}

fn potegowanie(){
    println!("Podaj podstawę potęgi: ");
    let mut text2 = String::new();
    io::stdin()
        .read_line(&mut text2)
        .expect("Błąd");
    let choose2:i8 = text2.trim().parse()
        .expect("Błąd");

    println!("Podaj wykładnik potęgi: ");
    let mut text3 = String::new();
    io::stdin()
        .read_line(&mut text3)
        .expect("Błąd");
    let choose3:i8 = text3.trim().parse()
        .expect("Błąd");

    let mut wynik = choose2.pow(choose3.try_into().unwrap());

    println!("Wynik: {}",wynik);
}

fn pierwiastkowanie(){
    println!("Podaj stopień pierwiastka:");
    let mut degree = String::new();
    io::stdin().read_line(&mut degree).expect("Błąd ");
    let degree: f64 = degree.trim().parse().expect("Błą");

    // Pobranie liczby od użytkownika
    println!("Podaj liczbę do pierwiastkowania:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Błąd");
    let number: f64 = number.trim().parse().expect("Błą");

    // Obliczenie pierwiastka
    let result = number.powf(1.0 / degree);

    // Wyświetlenie wyniku
    println!("WyniK: {}",result);
}
