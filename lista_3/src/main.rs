//Lista 3
mod poly;
use poly::Poly;

mod fig;
use fig::{Fig, Figura};
use std::f64::consts::PI;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use chrono::NaiveDate;

use std::fs::File;
use std::io::{Result, ErrorKind, Error};
use serde_json;

use flo_draw::*;
use flo_draw::canvas::*;



fn print_osoby(osoby: &Vec<Osoba>){
	for osoba in osoby {
		println!(
			"{} {} | Wzrost: {} cm | Waga: {} kg | Data urodzenia: {}",
			osoba.imie, osoba.nazwisko, osoba.wzrost, osoba.waga, osoba.data_urodzenia
		);
	}
	println!();
}

fn print_figury(figury: &Vec<Fig>) {
	println!("Figury:");
	for figura in figury {
		println!{"{}", figura};
	}
}


//Zadanie 2
fn fibonacci(n: usize) -> BigUint {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    for _ in 0..n {
        let temp = a + &b;
        a = b;
        b = temp;
    }
    b
}


//Zadanie 3
#[derive(Debug, Clone)]
pub struct Osoba {
	pub imie: String,
	pub nazwisko: String,
	pub wzrost: u32,
	pub waga: f64,
	pub data_urodzenia: String,
}

fn sort_by_name(osoby: &mut Vec<Osoba>) {
    osoby.sort_by_key(|osoba| (osoba.imie.clone(), osoba.nazwisko.clone()));
}

fn sort_by_age(osoby: &mut Vec<Osoba>) {
	osoby.sort_by_key(|osoba| {
		NaiveDate::parse_from_str(&osoba.data_urodzenia, "%d-%m-%Y").unwrap()
	});
}

fn age_range(osoby: &Vec<Osoba>, l: u32, p: u32) -> Vec<Osoba> {
	let mut nowe_osoby: Vec<Osoba> = Vec::new();

	for osoba in osoby {
		if osoba.wzrost >= l && osoba.wzrost <= p {
			nowe_osoby.push(osoba.clone());
		}
	}
	nowe_osoby
}

fn weigth(osoby: &Vec<Osoba>) -> (f64, f64) {
	let laczna_waga: f64 = osoby.iter().map(|osoba| osoba.waga).sum();
	let srednia_waga: f64 = if !osoby.is_empty() { laczna_waga / osoby.len() as f64 } else {0.0};

	(laczna_waga, srednia_waga)
}

fn sum_and_avarage<T>(figury: &Vec<Fig>, f: T) -> (f64, f64)
where 
	T: Fn(&Fig) -> f64,
{
	let suma: f64 = figury.iter().map(|fig| f(fig)).sum();
	let srednia = if !figury.is_empty() { suma / figury.len() as f64 } else { 0.0 };

	(suma, srednia)
}


//Zadanie 5
fn save(figury: &Vec<Fig>, filename: &str) -> Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer(file, &figury).map_err(|e| Error::new(ErrorKind::Other, e))?;
    Ok(())
}

fn load(filename: &str) -> Result<Vec<Fig>> {
    let file = File::open(filename)?;
    let figury: Vec<Fig> = serde_json::from_reader(file).map_err(|e| Error::new(ErrorKind::Other, e))?;
    Ok(figury)
}


fn main() {
	//Zadanie 1
	let p = Poly{ a: vec![1.0, 2.0, 3.0] };
	let q = Poly{ a: vec![1.0, 2.0] };

	let r = p.eval(q.clone());

	println!("Wielomian P: {}", p);
	println!("Wielomian Q: {}", q);
	println!("Wynik P(Q(x)): {}\n",r);


	//Zadanie 2
	let n = 1_000_000;
	let digit = 10_000;
	let fib_number = fibonacci(n);
    let fib_str = fib_number.to_string();
	let result = fib_str.chars().nth(digit - 1).unwrap();
    println!("{}-ta cyfra liczby Fibonacciego dla n = {} to: {}\n", digit, n, result);


	//Zadanie 3
	let mut osoby: Vec<Osoba> = vec![
		Osoba { imie: String::from("Jan"), nazwisko: String::from("Kowalski"), wzrost: 169, waga: 80.2, data_urodzenia: String::from("01-01-1990") },
		Osoba { imie: String::from("Anna"), nazwisko: String::from("Nowak"), wzrost: 165, waga: 60.5, data_urodzenia: String::from("12-04-1995") },
		Osoba { imie: String::from("Michal"), nazwisko: String::from("Wisniewski"), wzrost: 180, waga: 85.0, data_urodzenia: String::from("03-11-1988") },
		Osoba { imie: String::from("Katarzyna"), nazwisko: String::from("Zielinska"), wzrost: 170, waga: 63.3, data_urodzenia: String::from("25-06-2000") },
		Osoba { imie: String::from("Tomasz"), nazwisko: String::from("Wojcik"), wzrost: 190, waga: 92.7, data_urodzenia: String::from("19-07-1985") },

	];
	println!("Osoby: ");
	print_osoby(&osoby);

	//a)
	println!("Osoby posortowane wedlug imienia i nazwiska: ");
	sort_by_name(&mut osoby);
	print_osoby(&osoby);

	//b)
	println!("Osoby posortowane wedlug wieku: ");
	sort_by_age(&mut osoby);
	print_osoby(&osoby);

	//c)
	let (l, p) = (166, 180);
	println!("Osoby ze wzrostem z przedzialu od {} do {}", l, p);
	print_osoby(&age_range(&osoby, l, p));

	//d)
	let (laczna_waga, srednia_waga) = weigth(&osoby);
	println!("Waga wszystkich osob: {}, Srednia waga wszystkich osob: {}\n", laczna_waga, srednia_waga);

	//e)
	let figury: Vec<Fig> = vec![
        Fig::Kolo { r: 1.5 },
        Fig::Prost { a: 3.0, b: 2.0 },
        Fig::Kwadr { a: 4.0 },
        Fig::Romb { a: 3.0, alfa: PI / 3.0 },
    ];
	print_figury(&figury);
	let (suma, srednia) = sum_and_avarage(&figury, |f| f.pole());
    println!("Suma pol: {:.2}, Srednia pol: {:.2}\n", suma, srednia);


	//Zadanie 5
	if let Err(e) = save(&figury, "figury.json") {
        eprintln!("Blad zapisu: {}", e);
        return;
    }

    match load("figury.json") {
        Ok(loaded_figury) => {
            println!("Zaladowane figury:");
            for figura in loaded_figury {
                println!("{} - Pole: {:.3}, Obwod: {:.3}", figura, figura.pole(), figura.obwod());
            }
        }
        Err(e) => eprintln!("Blad ladowania: {}", e),
    }

	//Zadanie 6
	with_2d_graphics(move || {
		let canvas = create_canvas_window("Figury");

		let figury = figury.clone();

		canvas.draw(move |gc| {
			gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
			gc.canvas_height(600.0);
			gc.center_region(0.0, 0.0, 800.0, 600.0);

			let mut x_offset: f32 = 100.0;
			for fig in &figury {
				fig.paint(gc, x_offset, 300.0);
				x_offset += 200.0;
			}
		});
	});

}