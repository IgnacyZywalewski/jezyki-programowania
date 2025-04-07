//Lista 3
mod zadanie_1;
use zadanie_1::Poly;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use chrono::NaiveDate;


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

fn print_vector(osoby: &Vec<Osoba>){
	for osoba in osoby {
		println!(
			"{} {} | Wzrost: {} cm | Waga: {} kg | Data urodzenia: {}",
			osoba.imie, osoba.nazwisko, osoba.wzrost, osoba.waga, osoba.data_urodzenia
		);
	}
	println!();
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
		NaiveDate::parse_from_str(&osoba.data_urodzenia, "%Y-%m-%d").unwrap()
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


fn main() {
	//Zadanie 1
	let p = Poly{ a: vec![1.0, 2.0, 3.0] };
	let q = Poly{ a: vec![1.0, 2.0] };

	let r = p.eval(q.clone());

	println!("Wielomian P: {}", p);
	println!("Wielomian Q: {}", q);
	println!("Wynik P(Q(x)): {}",r);

	//Zadanie 2
	/*
	let n = 1_000_000;
	let fib_number = fibonacci(n);
    let fib_str = fib_number.to_string();
	let digit = 10_000;
	let result = fib_str.chars().nth(digit - 1).unwrap();
    println!("{}-ta cyfra liczby Fibonacciego dla n = {} to: {}", digit, n, result);*/

	//Zadanie 3
	let mut osoby: Vec<Osoba> = vec![
		Osoba { imie: String::from("Jan"), nazwisko: String::from("Kowalski"), wzrost: 169, waga: 80.2, data_urodzenia: String::from("1990-01-01") },
		Osoba { imie: String::from("Anna"), nazwisko: String::from("Nowak"), wzrost: 165, waga: 60.5, data_urodzenia: String::from("1995-04-12") },
		Osoba { imie: String::from("Michal"), nazwisko: String::from("Wisniewski"), wzrost: 180, waga: 85.0, data_urodzenia: String::from("1988-11-03") },
		Osoba { imie: String::from("Katarzyna"), nazwisko: String::from("Zielinska"), wzrost: 170, waga: 63.3, data_urodzenia: String::from("2000-06-25") },
		Osoba { imie: String::from("Tomasz"), nazwisko: String::from("Wojcik"), wzrost: 190, waga: 92.7, data_urodzenia: String::from("1985-07-19") },

	];

	println!("Osoby: ");
	print_vector(&osoby);

	//a)
	println!("Osoby posortowane wedlug imienia i nazwiska: ");
	sort_by_name(&mut osoby);
	print_vector(&osoby);

	//b)
	println!("Osoby posortowane wedlug wieku: ");
	sort_by_age(&mut osoby);
	print_vector(&osoby);

	//c)
	let (l, p) = (170, 180);
	println!("Osoby ze wzrostem z przedzialu od {} do {}", l, p);
	print_vector(&age_range(&osoby, l, p));

	//d)


	//e)

}