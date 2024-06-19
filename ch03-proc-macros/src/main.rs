use hello_world_macro::{Hello, HelloSimple};

#[derive(Hello)]
struct Target;

#[derive(HelloSimple)]
enum Pet {
    Cat,
}

fn main() {
    let ex = Target;
    ex.hello_world();

    let cat = Pet::Cat;
    cat.hello_world();
}
