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
    human_id("Paul-Emmanuel", 56, 178.3);
    let _x = {
        let price = 5;
        let qty = 10;
        // evaluate the last line
        // equal to 
        // return price * qty;
        price * qty
    };
    println!("Result is: {}", _x);
    // add(4,6);
    let y = add(4, 6);
    println!("Value of y is: {}", y);
    println!("Value from function 'add' is {}.", add(4,7));

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.78;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);

}


// Hoisting - can call function anywhere in your code
// a function variable shouild be written in snake case
// snake case: hello_world
// kebab case: hello-world
fn hello_world() {
    println!("Hello, Rust🦀!");
}

// you can insert input values
fn tell_height(height: i32){
    println!("My height is {} cm.", height);
}

// you can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

// function returning values
fn add(a:i32, b:i32) -> i32{
    a + b
}

// Expressions and Statements
// Expression: Anything that return a value.

// Statement: Anything that daoes not return a value.
// Almost all statement in Rust end with ;
// let y = let x = 10;
// 1 Variable declarations: let x = 5;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condition { /* code */ }
//  else { /* code */ }, while condition { /* code */ }, etc.

// Expression
//-----------
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}
// ({code})

// Final example
// BMI = weight(kg)/height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}