//This file contains examples and basic concepts of functions in Rust.
fn main() { //fn means function and this main function is where code execution starts. it is usually declared with empty brackets which returns a unit type (), but it can also accept parameters.
  println!("Hello Rust world!");
  another_function(); //We called another function here and it prints "This is another function".
}

fn another_function() { //Functions in Rust are declared outside the main function. Although functions are usually called in the main function, they can also be called in a different function.
  println!("This is another function!");
}// The curly brackets starts and ends the code block in functions.

//Code blocks in Rust.
fn my_function() { //variables and Functions in Rust are declared using lower snake_case with exception of constants which are decalared using upper snake_case.
let my_var = { //curly brackets starts a new code block and you can also declare a variable this way.
 let my_var = 500 + 100; //You can see we declared my_var twice, but this is code prints, why is that? Because this is another code block and the first my_var is in a seperate code block.
 my_var + 250 // this code adds my_var to 250 without an ending semicolon, so that the value can be returned and declared to the first my_var variable.
}; //The semicolon ends the code block without it the compiler prints an error message and if added to the code above it then the code block returns an empty bracket called unit type.
}

//Printing in Rust.
fn main() {
  let my_print = "Let's see what this prints"; //This is a string of type &str.
  println!("{}", my_print); //this prints, Let's see what this prints. Curly brackets does not always mean new code blocks in Rust, they are also used for printing.
  println!("{my_print}"); //curly brackets can also hold variables and print it.
  println!("This is also valid but {my_print}"); //Adding letters and characters in the println! macro is also valid Rust code. guess what this prints?
}
