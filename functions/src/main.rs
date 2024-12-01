// Functions
// Entry point

// If no main() function
// error[E0601]: `main` function not found in crate `functions`
fn hello() {
    println!("Hello, world!");
}

fn main() {
    hello();
    hello_world();
    tell_height(178);
}

// Hoisting - can call function anywhere in your code
// a function / variable shouild be written in snake case
// snake case: hello_world
// kebab case: hello-world
fn hello_world() {
    println!("Hello, Rust🦀!");
}

// you can inssert input values
fn tell_height(height: i32){
    println!("My height is {}", height);
}