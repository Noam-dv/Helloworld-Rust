struct Number {
    before: i32,
    number: i32,
    after: i32
}

fn main() {
    let number = Number {
        before: 31,
        number: 32,
        after: 33
    };

    println!("{:?}", number.before);
    println!("{:?}", number.number);
    println!("{:?}", number.after);
}