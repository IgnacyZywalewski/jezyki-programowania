//Zadanie przygotowujace na kolokwium

//Zadanie 9
fn policz(tab: &Vec<i32>) -> (i32, i32) {
    let (mut dodatnie, mut ujemne) = (0, 0);
    for &x in tab {
        if x > 0 {
            dodatnie += 1;
        }
        else if x < 0 {
            ujemne += 1;
        }
    }
    (dodatnie, ujemne)
}

//Zadanie 13
enum Trojmian{
    Ogolna { a: f64, b: f64, c: f64},
    Kanoniczna { a: f64, p: f64, q: f64},
    Iloczynowa { a: f64, x1: f64, x2: f64},
}


impl Trojmian {
    fn eval(&self, x: f64) -> f64 {
        match self {
            Trojmian::Ogolna { a, b, c } => a * x * x + b * x + c,
            Trojmian::Kanoniczna { a, p, q } => a * (x - p).powi(2) + q,
            Trojmian::Iloczynowa { a, x1, x2 } => a * (x - x1) * (x - x2),
        }
    }

    fn na_ogolna(&self) -> Trojmian {
        match *self {
            Trojmian::Ogolna { a, b, c } => Trojmian::Ogolna { a, b, c },
            Trojmian::Kanoniczna { a, p, q } => {
                let b = -2.0 * a * p;
                let c = a * p * p + q;
                Trojmian::Ogolna { a, b, c }
            }
            Trojmian::Iloczynowa { a, x1, x2 } => {
                let b = -a * (x1 + x2);
                let c = a * x1 * x2;
                Trojmian::Ogolna { a, b, c }
            }
        }
    }
    
}


fn main() {

    //Zadanie 1
    let mut i = 99;
    while i >= 1 {
        print!("{} ", i);
        i -= 2;
    }
    println!();

    let mut i = 99;
    loop {
        print!("{} ", i);
        if i == 1 {  
            break;
        }
        i -= 2;
    }
    println!();

    for i in (1..=99).rev().step_by(2) {
        print!("{} ", i);
    }
    println!();

    //Zadanie 2
    let (imie, wzrost, waga) = ("Jan", 180, 77.5);

    //Zadanie 3
    let ones: [i32; 100] = [1; 100];

    //Zadanie 4
    let box_of_ones: [[i32; 100]; 100] = [[1; 100]; 100];

    //Zadanie 5
    let a = 10;
    let b = 20;
    let c;
    if a > b {
        c = a;
    }
    else {
        c = b;
    }

    let c = if a > b { a } else { b };

    //Zadanie 6
    let max = if a > b {
        if a > c { a } else { c } 
    } 
    else {
        if b > c { b } else { c }
    };

    //Zadanie 7
    let tab: Vec<i32> = vec![1, -2, 3, -4];
    for &x in &tab {
        if x < 0 {
            print!("{} ", x);
        }
    }
    println!();

    //Zadanie 8
    let m = 2;
    let dni = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => panic!("Zly miesiac"),
    };

    //Zadanie 10
    struct Vec2D(f64, f64);
    let a = Vec2D(1.0, 2.0);
    let b = Vec2D(3.0, 4.0);
    let c = Vec2D(a.0 + b.0, a.1 + b.1);

    //Zadanie 11
    struct Trojkat {
        a: f64,
        b: f64,
        c: f64,
    }

    impl Trojkat{
        fn obwod(&self) -> f64 {
            self.a + self.b + self.c
        }
    }

    trait Obwod {
        fn obwod(&self) -> f64;
    }

    impl Obwod for Trojkat {
        fn obwod(&self) -> f64 {
            self.a + self.b + self.c
        }
    }

    //Zadanie 12
    let t = Trojkat { a: 3.0, b: 4.0, c: 5.0};
    let Trojkat{a, b, c} = t;
    let boki: Vec<f64> = vec![a, b, c];
    println!("Czy mozna zbudowac trojkat {}", boki[0] + boki[1] > boki[2] && boki[0] + boki[2] > boki[1] && boki[2] + boki[1] > boki[0]);

}