//Lista 3 Zadanie 4
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Osoba {
	pub imie: String,
	pub nazwisko: String,
	pub wzrost: u32,
	pub waga: f64,
	pub data_urodzenia: String,
}

fn print_osoby(osoby: &Vec<Osoba>){
	for osoba in osoby {
		println!(
			"{} {} | Wzrost: {} cm | Waga: {} kg | Data urodzenia: {}",
			osoba.imie, osoba.nazwisko, osoba.wzrost, osoba.waga, osoba.data_urodzenia
		);
	}
}

fn load(path: &str) -> Vec<Osoba> {
    if Path::new(path).exists() {
        let file = File::open(path).expect("Nie mozna otworzyc pliku.");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } 
    else {
        vec![]
    }
}

fn save(path: &str, osoby: &Vec<Osoba>) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Nie mozna zapisac do pliku.");
    serde_json::to_writer_pretty(file, osoby).expect("Blad zapisu do JSON.");
}

fn main() {	
	let path = "osoby.json";
    let mut osoby = load(path);

    loop {
        println!("\nCo chcesz zrobic? [list_all, add, remove, find, exit]:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "list_all" => {
                if osoby.is_empty() {
                    println!("Brak osob");
                }
                else {
                    print_osoby(&osoby);
                }
            }

            "add" => {
                let mut buf = String::new();

                println!("Imie: ");
                io::stdin().read_line(&mut buf).unwrap();
                let imie = buf.trim().to_string();
                buf.clear();

                println!("Nazwisko: ");
                io::stdin().read_line(&mut buf).unwrap();
                let nazwisko = buf.trim().to_string();
                buf.clear();

                println!("Wzrost (cm): ");
                io::stdin().read_line(&mut buf).unwrap();
                let wzrost: u32 = buf.trim().parse().unwrap_or(0);
                buf.clear();

                println!("Waga (kg): ");
                io::stdin().read_line(&mut buf).unwrap();
                let waga: f64 = buf.trim().parse().unwrap_or(0.0);
                buf.clear();

                println!("Data urodzenia (dd-mm-yyyy): ");
                io::stdin().read_line(&mut buf).unwrap();
                let data_urodzenia = buf.trim().to_string();

                osoby.push(Osoba {
                    imie,
                    nazwisko,
                    wzrost,
                    waga,
                    data_urodzenia,
                });

                save(path, &osoby);
                println!("Dodano osobe");
            }

            "remove" => {
                println!("Podaj nazwisko osoby do usuniecia: ");
                let mut nazwisko = String::new();
                io::stdin().read_line(&mut nazwisko).unwrap();
                let nazwisko = nazwisko.trim().to_lowercase();

                let before = osoby.len();
                osoby.retain(|o| o.nazwisko.to_lowercase() != nazwisko);
                let after = osoby.len();

                if before == after {
                    println!("Nie znaleziono osoby o podanym nazwisku");
                } 
                else {
                    println!("Usunieto osobe");
                    save(path, &osoby);
                }
            }

            "find" => {
                println!("Wprowadz fragment nazwiska: ");
                let mut fragment = String::new();
                io::stdin().read_line(&mut fragment).unwrap();
                let fragment = fragment.trim().to_lowercase();

                let znalezione: Vec<&Osoba> = osoby
                    .iter()
                    .filter(|o| o.nazwisko.to_lowercase().contains(&fragment))
                    .collect();

                if znalezione.is_empty() {
                    println!("Brak pasujacych osob");
                } 
                else {
                    for osoba in znalezione {
                        println!(
                            "{} {} | Wzrost: {} cm | Waga: {} kg | Data urodzenia: {}",
                            osoba.imie, osoba.nazwisko, osoba.wzrost, osoba.waga, osoba.data_urodzenia
                        );
                    }
                }
            }

            "exit" => {
                println!("Zamykam program");
                break;
            }

            _ => {
                println!("Nieznana komenda");
            }
        }
    }

}
