//Lista 1 bez zadania 5

mod zadanie_7;
use zadanie_7::Rectangle;

//Zadanie 2
fn nwd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nwd() {
        assert_eq!(nwd(56, 98), 14);
        assert_eq!(nwd(48, 18), 6);
        assert_eq!(nwd(101, 10), 1);
        assert_eq!(nwd(35, 70), 35);
        assert_eq!(nwd(27, 36), 9);
        assert_eq!(nwd(0, 5), 5);
    }
}

//Zadanie 3
use std::f64::consts::PI;

fn print_table() {
    println!("| {0:2} | {1:10} | {2:10} | {3:10} |", "Kat", "Sinus", "Cosinus", "Tangens");
    println!("|-----|------------|------------|------------|");

    for angle in 0..=45 {
        let radian = (angle as f64) * PI / 180.0;

        let sin_value = radian.sin();
        let cos_value = radian.cos();
        let tan_value = radian.tan();

        println!(
            "| {x:2}  | {s:10.8} | {c:10.8} | {t:10.8} |",
            x = angle,
            s = sin_value,
            c = cos_value,
            t = tan_value
        );
    }
}


//Zadanie 4
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn guessing_game() {
    println!("Zgadnij liczbe!");
    let secret_number = rand::rng().random_range(1..=100);

    loop {  
        println!("Wprowadz liczbe: ");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Bledny odczyt");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Twoja liczba {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mala!"),
            Ordering::Greater => println!("Za duza!"),
            Ordering::Equal => {
                println!("Zgadles!");
                break;
            }
        }
    }
}


//Zadanie 6
fn sum(t: &[i32]) -> i32{
    t.iter().sum()
}

fn max_ascend(t: &[i32]) -> &[i32]{
    if t.is_empty(){
        return &[];
    }

    let mut max_start = 0;
    let mut max_len = 1;
    let mut current_start = 0;
    let mut current_len = 1;

    for i in 1..t.len(){
        if t[i] > t[i - 1] {
            current_len += 1;
        }

        else{
            if current_len > max_len {
                max_start = current_start;
                max_len = current_len;
            }
            current_start = i;
            current_len = 1;
        }
    }

    if current_len > max_len {
        max_start = current_start;
        max_len = current_len;
    }

    &t[max_start..max_start + max_len]

}

fn accumulate(t: &mut [i32]){
    for i in 1..t.len() {
        t[i] += t[i - 1];
    }
}

fn main() {
    //Zadanie 2
    let a :i32 = 27;
    let b :i32 = 36;
    println!("Najwiekszy wspolny dzielnik {a} i {b} to {}\n", nwd(a, b));

    //Zadanie 3
    print_table();
    println!();

    //Zadanie 4
    guessing_game();
    println!();

    //Zadanie 6
    let mut numbers = [5, 2, 3, 4, 1];
    println!("Tablica {:?}", numbers);

    let result = sum(&numbers);
    println!("Suma tablicy {}", result);

    let ascend = max_ascend(&numbers);
    println!("Najdluzszy rosnacy podciag {:?}", ascend);

    accumulate(&mut numbers);
    println!("Tablica po akumulacji {:?}\n", numbers);

    
    //Zadanie 7
    let mut rect = Rectangle::new(1.0, 1.0, 4.0, 6.0);
    
    println!("Prostokat: {:?}", rect);

    let area = rect.area();
    println!("Pole: {}", area);

    let perimeter = rect.perimeter();
    println!("Obwod: {}", perimeter);
    
    rect.move_vec(2.0, 3.0);
    println!("Po przesunieciu o (2,3): {:?}", rect);

    rect.move_to(0.0, 5.0);
    println!("Po przesunieciu do punktu (0,5): {:?}", rect);
    
    rect.rotate_90();
    println!("Po obrocie o 90 stopni: {:?}", rect);
    
    rect.scale(1.5);
    println!("Po skalowaniu o 1.5x: {:?}", rect);
}