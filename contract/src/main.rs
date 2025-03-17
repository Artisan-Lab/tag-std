use my_macro::contract;

#[contract(ValidInt)]
fn example() {
    println!("Hello from example!");
}

fn main() {
    example();
}

