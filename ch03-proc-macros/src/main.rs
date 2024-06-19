use hello_world_macro::{Hello, HelloSimple, HelloVenial};

#[derive(Hello)]
struct Target;

#[derive(HelloSimple)]
enum Pet {
    Cat,
}

#[derive(HelloVenial)]
struct Other;

fn main() {
    let ex = Target;
    ex.hello_world();

    let cat = Pet::Cat;
    cat.hello_world();

    let other = Other;
    other.hello_world();
}
