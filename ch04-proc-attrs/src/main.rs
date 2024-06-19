use make_public_macro::{public, public_2, public_3};

#[derive(Debug)]
#[public]
struct Example {
    first: String,
    second: u32,
}

#[derive(Debug)]
#[public_2]
struct Example2 {
    foo: String,
    bar: Vec<String>,
}

#[derive(Debug)]
#[public_3]
struct Example3 {
    foo: String,
}

fn main() {
    let ex = Example {
        first: String::from("Collin"),
        second: 48,
    };
    let _ = dbg!(ex);
    println!("Hello, world 04!");
}
