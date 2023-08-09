#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Special(String),
    Petikacka,
    Pajcka,
    Papirovka(u32),
}

fn main() {
    // let kojn = Coin::Papirovka(100);
    let kojn = Coin::Special("ahoj".to_string());
    match &kojn {
        Coin::Special(message) => {
            println!("special coin found: {}", message)
        }
        Coin::Petikacka => {
            println!("5 Kc")
        }
        Coin::Pajcka => {
            println!("50 Kc")
        }
        Coin::Papirovka(cena) => {
            println!("papirovka: {cena} Kc")
        }
        // Coin::Papirovka => println!("papirovka: {cena} Kc"), // cannot do this
    }

    println!("{:?}", kojn);
}
