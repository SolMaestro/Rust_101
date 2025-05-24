//This file contains code snippets and examples of Control Flow programs, Loops and Match statements.
//if, else, else if control flow
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

let other_number = 505;
if other_number > 500 { //the greater than symbol is also among the relational operators used to evaluate control flow programs in Rust. just like its name, it checks and evaluates variables and goes ahead to print an output if the condition is true.
  println!("My number is greater than 500"); //this prints `My number is greater than 500` because it actually is.
} else if other_number < 200 { //just like the greater sign, the less than sign also evalutes an expression.
  println!("My number is lesser than 200"); //This doesnt print because its false, and even if its true, Rust only prints the first true expression and skips the rest.
} else { //the else expression is optional in a code block especially when two strict values are compared which would definitely return a true or false, but its good practice to always include it incase of any unexpected behaviour in the comparisons.
  println!("None of my numbers is greater or lesser!"); //As usual this line prints if all expressions above evaluates to false.
}

let my_number = 200;
let last_number = 150;
if my_number == 200 && last_number == 150 { //The double ampersands `&&` are Logical operators used to combine two or more expressions resulting in a true or false value. This called AND.
  println!("My both numbers correspond"); //This line gets printed if only the two expressions being compared are true. If only one results to false then the whole expressions evaluates to false which wouldnt print this line.
} else if my_number == 150 || last_number == 150 { //The double slash `||`is also a logical operator which means OR. it simply checks if either of the operands is true and if any of them is true then it executes a given command such as printing a line of code or other complex operations.
  println!("Any of my number corresponds to 150"); //even though either of these expressions evaluates to true, this line doesnt print because the evaluation on the line above is also true, so Rust simply skips the rest.
} else {
  println!("Next number please!");
}

let married = true; //Booleans are also a good way to control the flow of programs in Rust. 
let exhausted = false; //Boolean values are explicitly assigned to these variables.
if married && exhausted { //Using the `AND` logical operator the expression evalutes the two operands above.
  println!("I am married and exhausted"); //This does not print because one of the conditions is false. Remember they both need to be true in for this line to print.
} else if married || exhausted { //Since this is an `OR` expression, the code prints the next line. Either of them needs to be true.
  println!("Because i'm either married or exhausted");
} else if !married && exhausted { //see the exclamation mark? That's also another logical operator called `!NOT`. It changes true to false by just adding the exclamation mark before the true variable.
  println!("Guess the output"); //Since both operands are false and it's an `AND` operator. This line should print if other expressions were false.
} else {
  println!("I am neither married nor exhausted");
}