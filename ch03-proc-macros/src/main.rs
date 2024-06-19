use hello_world_macro::Hello;

#[derive(Hello)]
struct Example;

fn main() {
    let ex = Example;
    ex.hello_world();
}
