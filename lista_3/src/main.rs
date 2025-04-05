//Lista 3
mod poly;
use poly::Poly;

use num_bigint::BigUint;
use num_traits::{One, Zero};

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

fn main() {
	//Zadanie 1
	let p = Poly{ a: vec![1.0, 2.0, 3.0] };
	let q = Poly{ a: vec![1.0, 2.0] };

	let r = p.eval(q.clone());

	println!("Wielomian P: {}", p);
	println!("Wielomian Q: {}", q);
	println!("Wynik P(Q(x)): {}",r);

	//Zadanie 2
	let n = 1_000_000;
	let fib_number = fibonacci(n);
    let fib_str = fib_number.to_string();
	let digit = 10_000;
	let result = fib_str.chars().nth(digit - 1).unwrap();
    println!("{}-ta cyfra liczby Fibonacciego dla n = {} to: {}", digit, n, result);

}