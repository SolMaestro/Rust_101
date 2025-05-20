//this file contains basic concepts and primitive code examples.

fn main() {
  println!("Hello Rust World!");     /*fn means function, main() is the function that starts a program in Rust, in this example it does not take an argument and retuns a unit type ()*/ 
 }                                   //This prints "Hello Rust World!"

//Primitive types in Rust
let my_integer: u8 = 10; //Integers in rust can be signed or unsigned and this statement declares an unsigned 1 byte(8 bits = 1 byte).
let my_signed: i8 = 25; //declaration of a signed 1 byte integer.
let my_number = 500; //Rust can use type inference to note the type without declaring it explicitly, with the default type being i32(signed 32 bits).

let my_float: f32 = 25.0; //declaration of a float variable, the f32 type has single precision.
let other_float = 50.0; //this is the default type inferenced by Rust, an f64 that has double precision.

let my_bool = true; // Like most other programming languages, this uevalutes a value if the condition is true.
let other_bool = false // like true, this evalutes a value if the condition is false. Bools are mostly used with conditional control flows.

let my_char = 'A' //Char is a primitive alphabetic type, you can represent letters including korean and chinese characters, emojis. 
                  //PS. They are declared using single quotes as opposed to strings which use double quotes.

//Arithmetic Operations in Rust
let sum = 50 + 150; //Addition, This prints 200. the type is i32 because that is the default rust type inference for integers.

let difference = 9.5 - 4.3; //Subtraction, this prints 5.2. The type is f64 as inferenced by Rust for float pointing numbers.

let product = 500_000_000 * 200_000_000; /*Multiplication, this particular code does not compile because the result of the expression cannot fit into an i32,
                                          so it prints an error message and exits. In short, this error is called arithmetic overflow. You can solve this by explicitly adjusting
                                          the i32 to i64 or i128  as the case may be.*/

let quotient = 79.1 / 12.3 //Division, this prints 6.430894308943088. To reduce the precision values, the variable type can be explictly annonated with f32.

let remainder = 73 % 5; //This is a modulo operation which finds a remainder after dividing one number by another. This code prints 3.




//Shadowing in Rust: The concept of shadowing is when a varible blocks another variable with same name.
fn main() {
  let shadow =  50_000_000; //Yes! you can use underscores to seperate number values, to make it readable.
  let shadow = 100_000_000;

  println!("My shadow is {shadow}"); //This code prints 100_000_000, because it shadows the first variable shadow.
  println!("My shadow is also {}", shadow); //You can also print the variables using this format.
}

//Mutability in Rust: By default, the values in variables in rust cannot be changed except you declare it as mutable using the keyword "mut".
fn main() {
  let my_value: u8 = 155;
  my_value = 255;
  println!("My value is {}", my_value); /*This code block prints an error by the compiler because you can't change the values without declaring it as mutable.
                                         Just add mut between let and my_value, the code should now print without errors*/

  let mut my_string = "My name is Rust"; //This string variable is declared as mutable by using the keyword mut. like other types in Rust, strings are inferenced as &str (&'static str) by default.
  my_string = 555;
  println!("My string is {my_string}"); //Even though my_string was declared as mutable by using the keyword "mut", this code also prints an error by the compiler because the types are misplaced. Although there are methods you can use to switch between types
}
