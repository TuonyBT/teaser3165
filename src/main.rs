use std::collections::HashMap;

fn main() {

//  Scalpel approach applies assertions derived from analysis of the problem, which renders a unique solution
//  Assertions are as follow:

//  Let sides of rectangle equal a and b, a > b; draw the rectangle on x-y axes with long side on Ox->
//  For integer perimeters to have a ratio of (28 - 5)/28, perimeter of rectangle must be divisible by 28, so
//  2(a+b) == k*28, i.e. a+b = k*14, where k is an integer
//  We are told that a < 50, so a+b < 100, so k <= 100//14 = 7, also if a > b, a > 7 so k < 7

//  The fold line passess through the centre of the rectangle, at right-angles to the diagonal between the corners that meet
//  If the line passes through the long side (x-axis) at x = s, its equation is y = s - ax/b
//  We can therefore derive s knowing it also passes through the centre (a/2, b/2): s = (a^2 + b^2) / 2 / a
//  Define r = a - s
//  The length of this fold line (de) is the hypotenuse of the triangle made from projecting the line on to the x-axis, i.e. with
//  perpendicular sides b and (a - 2 * r)

//  The sides of the pentagon have lengths b, r (from rectangle), b, r (from reflected corner) and de
//  Therefore de and r (and therefore s) must be integers
//  Substituting b = 14k - a into s, and simplifying, tells us that for s to be an integer, (2.7.7.k.k)/a must be an integer
//  If we define a = k.a' and b = k.b', a' + b' = 14, and 2.7.7.k/a must be an integer
//  But if a' + b' = 14 and a>b, 7<a'<14, so so 7 cannot be a factor of a'; also a' <> 2 and a' <> k because k < 7
//  So a' = 2k and a = 2k * k

//  a = 2k^2 < 50, so k^2 < 25, so k<5
//  b = k * b' < a, so k * (14 - a') < a, which requires that k > 7 / 2, so k >= 4
//  So k = 4

//  whence:
    for k in 4i32..5 {
        let a = k * k * 2;
        let b = k * 14 - a;
        let s = ((a.pow(2) + b.pow(2)) as f64) / a as f64 / 2.0; 
        let r = a as f64 - s;
        let de_sq = (b as f64).powf(2.0) + (a as f64 - 2.0 * r).powf(2.0);
        let de = (de_sq as f64).powf(0.5) ;
        let perimeter_lost = (2.0 * s - de) * 14.0 / (a + b) as f64;
        println!("Scalpel approach");
        println!("a: {}, b: {}, m: {}, s: {}, r: {}, de: {}, perimeter lost: {}", a, b, k, s, r, de, perimeter_lost);
        println!();
       }



//  Sledgehammer approach loops over all values of a, b where a < 50 and b < a
//  Calculates the lost perimeter value and stores it in a dictionary indexed by the
//  assertions applied in the scalpel solution, viz:
//  a = 2m^2, (a + b) = 14*k, (k, m integers), k = m = 4.
//  (if any intermediate value that is required to be an integer is not, store this value in lieu of lost perimeter value)

    let mut results = HashMap::<([bool; 2], i32), Vec<f64>>::new();

    for a in 2i32..50 {
        let mut sq: bool = false;
        let m = (a as f64 / 2.0).powf(0.5);
        if m % 1.0 == 0.0 {
            sq = true;
        } 
        for b in 1 .. a {
            let mut mult: bool = false;
            let mut k = 0i32;
            let n = a + b;
            if n % 14 == 0 {
                mult = true;
                k = n / 14;
            }

            let s = ((a.pow(2) + b.pow(2)) as f64) / a as f64 / 2.0; 
            if s % 1.0 == 0.0 {
                let r = a - s as i32;
                let de_sq = b.pow(2) + (a - 2 * r).pow(2);
                let de = (de_sq as f64).powf(0.5) ;
                if de % 1.0 == 0.0 {
                    let perimeter_lost = (2.0 * s - de) * 14.0 / (a + b) as f64;
                    results.entry(([sq, mult], k)).or_insert(Vec::<f64>::new()).push(perimeter_lost);
                }
                else {
                    results.entry(([sq, mult], k)).or_insert(Vec::<f64>::new()).push(de);
                } 
            }
            else {
                results.entry(([sq, mult], k)).or_insert(Vec::<f64>::new()).push(s);
            } 
        }
    }

    println!("Sledgehammer approach");

    for (k, v) in results {
        println!("a = 2m^2: {}, (a + b) = 14*k {}, m: {}: no of solutions {}", k.0[0], k.0[1], k.1, v.len());
        if k.0 == [true, true] {
            println!("Solutions are: {:?}", v);
        }
        println!("Solutions with whole number perimeters {:?}", v.iter().filter(|&p| p % 1.0 == 0.0).collect::<Vec<&f64>>());
        println!();
    }

}
