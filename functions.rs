//This file contains examples and basic concepts of functions in Rust.
fn main() { //fn means function and this main function is where code execution starts. it is usually declared with empty brackets which returns a unit type (), but it can also accept parameters.
  println!("Hello Rust world!");
  another_function(); //We called another function here and it prints "This is another function".
}

fn another_function() { //Functions in Rust are declared outside the main function. Although functions are usually called in the main function, they can also be called in a different function.
  println!("This is another function!");
}// The curly brackets starts and ends the code block in functions.
