//This file contains code snippets and examples of Control Flow programs, Loops and Match statements.
fn main() {
  let my_number = 255;
  if my_number == 256 { //The if expression is used to control the flow of programs, depending on conditions. this code simply checks if my_number is equal to 256.
    println!("My number is 255"); //This code prints nothing because my_number is not equal to 256 and also because the if condition evaluates to false.
  } //end of if block.
}

fn main() {
  let my_number = 255;
  if my_number == 256 {
    println!("My number is 255");
  } else {  //The programmer can also use the optional else expression to provide an alternative message incase the if expression evaluates to false.
    println!("My number is not equal to 255"); //this prints `My number is not equal to 255` since the above if expression is false.
  } //end of else block.
}

fn main() {
  let my_number = 255;
  if my_number == 256 {
    println!("My number is 255");
  } else if my_number == 255 { //There's also an else if expression that can be used to evaluate conditionals. This also checks if condition is true or false.
    println!("My number is finally equal to 255"); //This prints `My number is finally equal to 255`, because it's the only conditional that evaluates to true.
  } else if my_number == 100 { //Unlike the if and else expressions, The programmer can use the else if expression as many times as needed in a single control flow block.
    println!("My number equals 100"); //This code prints nothing.
  } else if my_number == 255 { //This expression also evaluates to true but it doesnt print, why? this is because Rust skips other expressions after getting a true value.
    println!("Again my number is 255!"); //This doesnt print because of the reason stated above.
  } else {
    println!("Let's move to another number"); //Rust only prints this if all other expressions evaluates to false.
  }
}

