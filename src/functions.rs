use std::f64::consts; // for pi

fn round(unrounded:f64, precision:Option<u32>) -> f64 {
    match precision {
        Some(p) => {
            let power = 10_i32.pow(p);
            (unrounded * power as f64).round()/power as f64
        }
        None => unrounded.round(),
    }
}

fn main() {
    let num = consts::PI;
    let rounded = round(num, None);
    let rounded_decimal = round(num, Some(1));
    let rounded_decimals = round(num, Some(2));

    println!("number {}", num);
    println!("rounded to whole {}", rounded);
    println!("rounded with 1 decimal place {}", rounded_decimal);
    println!("rounded with 2 decimal places {}", rounded_decimals);
}