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
                  //PS. They are declared using single quotes as opposed to strings which use double qoutes.



//Shadowing in Rust: The concept of shadowing is when a varible blocks another variable with same name.
fn main() {
  let shadow =  50_000_000; //Yes! you can use underscores to seperate number values, to make it readable.
  let shadow = 100_000_000;

  println!("My shadow is {shadow}"); //This code prints 100_000_000, because it shadows the first "shadow" variable.
  println!("My shadow is also {}", shadow); //You can also print the code using this format.
}
