// This file contains examples and basic concepts of functions in Rust.

fn main() {                                       // fn means function and this main function is where code execution starts. it is usually declared with empty brackets which returns a unit type (), but it can also accept parameters.
  println!("Hello Rust world!");
  another_function();                             // We called another function here and it prints "This is another function".
}

fn another_function() {                             // Functions in Rust are declared outside the main function. Although functions are usually called in the main function, they can also be called in a different function.
  println!("This is another function!");
}                                                   // The curly brackets starts and ends the code block in functions.

// Code blocks in Rust.

fn my_function() {                                    // Variables and Functions in Rust are declared using lower snake_case with exception of constants which are decalared using upper snake_case.
let my_var = {                                        // Curly brackets starts a new code block and you can also declare a variable this way.
 let my_var = 500 + 100;                              // You can see we declared my_var twice, but this is code prints, why is that? Because this is another code block and the first my_var is in a seperate code block.
 my_var + 250                                         // This code adds my_var to 250 without an ending semicolon, so that the value can be returned and declared to the first my_var variable.
};                                                    // The semicolon ends the code block without it the compiler prints an error message and if added to the code above it then the code block returns an empty bracket called unit type.
}

// Printing in Rust.

fn main() {
  let my_print = "Let's see what this prints";                // This is a string of type &str.
  println!("{}", my_print);                                   // This prints, Let's see what this prints. Curly brackets does not always mean new code blocks in Rust, they are also used for printing.
  println!("{my_print}");                                     // Curly brackets can also hold variables and print it.
  println!("This is also valid & good! but {my_print}.");     // Adding letters and characters in the println! macro is also valid Rust code. guess what this prints?
}

fn main() {
  print!("\tThis adds a tab and \nPrints on a new line");       // Adding /t to the print! macro adds a tab to the line and \n adds a new line to the print. note that the macro here is print!.
  println!("Println! and not print!");                          //printing with println! macro automatically prints on a new line while the print! macro prints on same line. 
}

//Function parameters.
fn my_function(x: i32) { //This function accepts a parameter which we declared as an i32 but it could be anything as long as it corresponds with the argument passed to it.
println!("The value of the parameter is {}", x);
}

fn other_function(x: u16, y: i32) { //The function above holds one parameter but it's also valid to declare two or more parameters in a function as long as you explicitly declare the type and pass the corresponding arguments correctly.
  println!("functions can hold more parameters and are seperated by commas like {} and {}", x , y);
}

fn string_parameter(name: &str) { //Functions can also hold strings and any other valid type as parameters in Rust.
  println!("strings can also be in function parameters like {name}.");
}

fn main() {
  my_function(5); //5 is passed here as an argument to the function and it corresponds to the i32 declared in my_function.
  other_function(55, 600); //functions can hold more than 1 argument as long as the parameters is specified when declaring the function.
  string_parameter("Rust Lang"); //This prints, strings can also be in function parameters like Rust Lang.
}

//Functions with Return Values
fn return_two() -> i32 { //Functions can return values to the code that calls them. the programmer declares their type after an arrow ->
 2  //this value ends without a semicolon since it returns the value 2.
}

fn early() -> u16 {
  return 500; //Rust returns early when the programmer uses the keyword "return" before a value of the specified return type.
  let value = 500 + 200; //Statements and expressions after the return keyword is unreachable.
  let another = 700 + 100; //Lastly, the compiler will warn of these unused variables before printing the return value.
}

fn main() {
  let my_return = return_two(); //Since the return_two function returns a variable, its totally valid to initialize a variable with the function.
  println!("The value of my_return is {}", my_return); //This line prints, The value of my_return is 2.

  let my_early = early();
  println!("We returned early with the value {}", my_early); //This prints, We returned early with the value 500.

  let addition = add_operation(20, 20);
  println!("The result of the addition is {}", addition); //This prints, The result of the addition is 240.
}

//Arithemetic operations with Functions and their Return values.
fn add_operation(x: i32, y: i32) -> i32 { //This function accepts 2 parameters and returns a value.
let add = x + y + 200; //arithemetic operation with the parameters.
add //return the variable without a semicolon.
}
