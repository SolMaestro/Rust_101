//this file contains basic concepts and primitive code examples.

fn main() {
  println!("Hello Rust World!"); /*fn means function, main() is the function that starts a program in Rust, 
                                  in this example it does not take an argument and retuns a unit type ()*/
} //This prints "Hello Rust World!"

//Primitive types in Rust

let my_integer: u8 = 10; //Integers in rust can be signed or unsigned and this code declares an unsigned 1 byte(8 bits = 1 byte).
let my_signed: i8 = 25; //declaration of a signed 1 byte integer.
let my_number = 500; //Rust can use type inference to note the type without declaring it explicitly, with the default type being i32.