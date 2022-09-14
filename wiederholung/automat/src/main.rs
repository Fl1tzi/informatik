use std::env;
use std::cmp::Ordering;

fn geldsumme_berechnen(geldsumme: u32) {
    let mut restsumme: u32 = geldsumme;
    let mut rueckgabe: Vec<u32> = Vec::new();
    while restsumme > 0 {
        for geld in [50000, 20000, 100000, 5000, 2000, 1000, 100, 50, 20, 10, 5, 2, 1] {
            match geld.cmp(&restsumme) {
                Ordering::Equal | Ordering::Less => {
                    restsumme = restsumme - geld;
                    rueckgabe.push(geld);
                    break;
                },
                Ordering::Greater => {continue},
            }
        }
    }
    println!("Rückgabe: {:?}", rueckgabe);

}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let geldsumme = || -> Result<u32, std::num::ParseIntError> {
                let geldsumme: u32 = args[1].parse()?;
                Ok(geldsumme)
            };
            match geldsumme() {
                Ok(v) => geldsumme_berechnen(v),
                Err(_e) => println!("Du musst Zahlen angeben, welche nicht kleiner als null sind.")
            }
        }
        _ => {
            println!("Nutzung: [Geld in c]");
            return;
        }
    }
}
