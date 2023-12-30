fn main() {
    println!("Printing in Rust:");

    let _name:&str = "noam";
    let _country:&str = "singapore";
    println!("my name is {} and im from {}", _name, _country);
    println!("my name is {name} and im from {country}", name = "Noam", country = "Singapore");

    let _n1 = 1;
    let _n2 = 5;
    println!("multiple of {} and {} is {}.", _n1, _n2, _n1 * _n2);

    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    print!("same");
    println!(" line");

    let pi = 3.1415;
    println!("{:.3}", pi); // limit decimal points (?)
}