fn main() {
    // Compound Data Types
    // array, tuples, slices and strings (slice string)

    // Arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    // error[E0277]: `[i32; 5]` doesn't implement `std::fmt::Display`
    // println!("Number Array: {}", numbers);
    println!("Number Array: {:?}", numbers);

    // let mix = [1,2,"apple",true];
    // error[E0308]: mismatched types
    // println!("Mix Array: {:?}", mix);
    let fruits: [&str;3] = ["Apple","Banana","Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    // Tuples
    // let human: (String, i32, bool) = ("Alice", 30, false);
    // error[E0308]: mismatched types
    let human = ("Alice", 30, false);
    println!("Human Tupple with String Slice: {:?}", human);
    // We need to convert String Slice Alice to String
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tupple with String: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tupple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);
    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);
    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // String [ growable, mulable, owned string type ]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    // Add mut to change to mutable variable
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    println!("Slice Value: {}", slice);
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}

// fn print(){
    // error[E0425]: cannot find value `slice` in this scope
    // println!("SLICE: {}", slice);
// }