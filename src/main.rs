use std::collections::HashMap;

fn main() {

    let mut results = HashMap::<[bool; 2], Vec<f64>>::new();

    for a in 2i32..50 {
        let mut sq: bool = false;
        let m = (a as f64 / 2.0).powf(0.5);
        if m % 1.0 == 0.0 {
            sq = true;
        } 
        for b in 1 .. a {
            let mut mult: bool = false;
            let m = a + b;
            if m % 14 == 0 {
                mult = true;
            } 
            let s = ((a.pow(2) + b.pow(2)) as f64) / a as f64 / 2.0; 
            if s % 1.0 == 0.0 {
                let r = a - s as i32;
                let de_sq = b.pow(2) + (a - 2 * r).pow(2);
                let de = (de_sq as f64).powf(0.5) ;
                if de % 1.0 == 0.0 {
                    let perimeter_lost = (2.0 * s - de) * 14.0 / (a + b) as f64;
                    results.entry([sq, mult]).or_insert(Vec::<f64>::new()).push(perimeter_lost);
                }
                else {
                    results.entry([sq, mult]).or_insert(Vec::<f64>::new()).push(de);
                } 
            }
            else {
                results.entry([sq, mult]).or_insert(Vec::<f64>::new()).push(s);
            } 
        }
    }

    for (k, v) in results {
        println!("a = 2m^2: {}, (a + b) = 14*k {}: no of solutions {}", k[0], k[1], v.len());
        if k == [true, true] {
            println!("Solutions are: {:?}", v);
        }
        println!("Solutions with whole number perimeters {:?}", v.iter().filter(|&p| p % 1.0 == 0.0).collect::<Vec<&f64>>());
        println!();
    }

    for m in 4i32..5 {
        let a = m * m * 2;
        let b = m * 14 - a;
        let s = ((a.pow(2) + b.pow(2)) as f64) / a as f64 / 2.0; 
        let r = a - s as i32;
        let de_sq = b.pow(2) + (a - 2 * r).pow(2);
        let de = (de_sq as f64).powf(0.5) ;
        let perimeter_lost = (2.0 * s - de) * 14.0 / (a + b) as f64;



        println!("a: {}, b: {}, m: {}, s: {}, r: {}, de: {}, perimeter lost: {}", a, b, m, s, r, de, perimeter_lost);
    }
}
