// This file contains code snippets and examples of Control Flow programs, Loops and Match statements.
// if, else, else if control flow

fn main() {
  let my_number = 255;
  if my_number == 256 {                           // The if expression is used to control the flow of programs, depending on conditions. this code simply checks if my_number is equal to 256.
    println!("My number is 255");                 // This code doesnt execute because my_number is not equal to 256 and also because the if condition evaluates to false.
  } //end of if block.
}

fn main() {
  let my_number = 255;
  if my_number == 256 {
    println!("My number is 255");
  } else {                                                    // The programmer can also use the optional else expression to provide an alternative message incase the if expression evaluates to false.
    println!("My number is not equal to 255");                // This prints `My number is not equal to 255` since the above if expression is false.
  } // End of else block.
}

fn main() {
  let my_number = 255;
  if my_number == 256 {
    println!("My number is 255");
  } else if my_number == 255 {                                  // There's also an else if expression that can be used to evaluate conditionals. This also checks if condition is true or false.
    println!("My number is finally equal to 255");              // This prints `My number is finally equal to 255`, because it's the only conditional that evaluates to true.
  } else if my_number == 100 {                                  // Unlike the if and else expressions, The programmer can use the else if expression as many times as needed in a single control flow block.
    println!("My number equals 100");                           // This code prints nothing.
  } else if my_number == 255 {                                  // This expression also evaluates to true but it doesnt print, why? this is because Rust skips other expressions after getting a true value.
    println!("Again my number is 255!");                        // This doesnt print because of the reason stated above.
  } else {
    println!("Let's move to another number");                   // Rust only prints this if all other expressions evaluates to false.
  }
}

let other_number = 505;
if other_number > 500 {                                         // The greater than symbol is also among the relational operators used to evaluate control flow programs in Rust. just like its name, it checks and evaluates variables and goes ahead to print an output if the condition is true.
  println!("My number is greater than 500");                    // This prints `My number is greater than 500` because it actually is.
} else if other_number < 200 {                                  // Just like the greater sign, the less than sign also evalutes an expression.
  println!("My number is lesser than 200");                     // This doesnt print because its false, and even if its true, Rust only prints the first true expression and skips the rest.
} else {                                                        // The else expression is optional in a code block especially when two strict values are compared which would definitely return a true or false, but its good practice to always include it incase of any unexpected behaviour in the comparisons.
  println!("None of my numbers is greater or lesser!");         // As usual this line prints if all expressions above evaluates to false.
}

fn main() {
  let modulo = 40;
  if modulo % 10 == 0 {                                           // The modulo operations are valid in Rust and helpful in control flow expressions. The modulo operation finds the remainder of a number when divided by another number. if the operation evaluates to true then Rust performs the required operation.
    println!("This operation is correct");                        // Since the operation above is true, the compiler prints this line.
  } else {                                                        // If not, it executes this code block as usual.
    println!("That's a wrong operation");
  }
}

let my_number = 200;
let last_number = 150;
if my_number == 200 && last_number == 150 {                       // The double ampersands `&&` are Logical operators used to combine two or more expressions resulting in a true or false value. This called AND.
  println!("My both numbers correspond");                         // This line gets printed if only the two expressions being compared are true. If only one results to false then the whole expressions evaluates to false which wouldnt print this line.
} else if my_number == 150 || last_number == 150 {                // The double slash `||`is also a logical operator which means OR. it simply checks if either of the operands is true and if any of them is true then it executes a given command such as printing a line of code or other complex operations.
  println!("Any of my number corresponds to 150");                // Even though either of these expressions evaluates to true, this line doesnt print because the evaluation on the line above is also true, so Rust simply skips the rest.
} else {
  println!("Next number please!");
}

let married = true;                                               // Booleans are also a good way to control the flow of programs in Rust. 
let exhausted = false;                                            // Boolean values are explicitly assigned to these variables.
if married && exhausted {                                         // Using the `AND` logical operator the expression evalutes the two operands above.
  println!("I am married and exhausted");                         // This does not print because one of the conditions is false. Remember they both need to be true in for this line to print.
} else if married || exhausted {                                  // Since this is an `OR` expression, the code prints the next line. Either of them needs to be true.
  println!("Because i'm either married or exhausted");
} else if !married && exhausted {                                 // See the exclamation mark? That's also another logical operator called `!NOT`. It changes true to false by just adding the exclamation mark before the true variable.
  println!("Guess the output");                                   // Since both operands are false and it's an `AND` operator. This line should print if other expressions were false.
} else {
  println!("I am neither married nor exhausted");
}

let not_equal = 800;
if not_equal != 500 {                                               // There's also a NOT operator that is also used in control flows. This simply translates to if not_equal is NOT equal to an integer or variable as the case may be, then perform some action.
  println!("Number was something other than 500");                  // The code executes because the expression is true.
} else {                                                            // If not, it executes the next line.
  println!("Number is {not_equal}");
}

let condition = false;
let assign = if condition {                             // The programmer can also assign a value to a variable using control flows in Rust. 
  100 
} else {                                                // Since the expression above evaluates to false Rust assigns the value 200 to the variable.
  200                                                   // Remember that to return a value in Rust, the programmer excludes the semi-colon. Also remember that the return values must be of same type, if we mismatch the types like returning a &str with an i32, the compiler will print an error message.
};                                                      // The semi-colon goes here instead, the end of the code block.
println!("Assign holds the value {}", assign);          // This prints `Assign holds the value 200`

//LOOPS
fn main() {
  loop {                                            // Loops are just simple ways to make a code print repeatedly till you tell it to stop.
    println!("I just keep printing");               // This line keeps printing till we command it to stop by using the CTRL + C key.
  }
}

fn main() {
  let mut counter = 0;                              // Declares a mutable variable named counter.
  loop {                                            // Fortunately, Rust allows the programmer to explictly tell the compiler when to breakout of the code by using the keyword `break`.
    counter += 1;                                   // Keeps adding 1 to the counter variable.
    println!("The counter is now {counter}");       // Prints a line that tells the programmer about the counter.
   if counter == 5 {                                // The programmer uses an if expression to determine when to breakout of the code.
    break;                                          // Finally, the keyword `break`, breaks out of the code immediately the condition is met.
   }
  }
}

fn main() {
  let mut counter1 = 0;
  let mut counter2 = 0;
  println!("Now entering first loop");
  'first_loop: loop {                                   // Rust allows the programmer to name a loop, which is helpful when you are in a loop that is inside another loop. This is possible by using a tick ` before the name.
    counter1 += 1;
    println!("First loop is now {counter1}");
    if counter1 == 10 {
        println!("Now entering second loop!");
    'second_loop: loop {                                // This line starts a second loop inside the first loop.
      counter2 += 1;
      println!("Second loop is now {counter2}");
      if counter2 == 10 {
        break 'first_loop;                              // Lastly, this line exits the loop once the second loop counts to 10;
      } else if counter2 == 20 {                        // The compiler never executes this line because the line above breaks out of the entire loop. Remember we're still inside the first loop.
          break 'second_loop;
      }
      }
    }
    }
  }

  // While loop
  let mut counter = 0;
  while counter < 5 {                                     // The while loop is another loop that continues while a condition is true, For each loop, Rust will check whether it is still true. If it becomes false, Rust will stop the loop.
    counter += 1;                                         // This line adds 1 to the counter till counter gets to 5 and turns the while expression to false so the loop stops.
    println!("Counter is now {counter}");                 // Run this code to see the outcome.
  }

  // for loop
  fn main() {
    for number in 0..3 {                                    // A for loop lets the programmer tell Rust what to do each time. Unlike the while loop that checks if a condition is true, the for loop stops after a certain number of times. Also the for loop use ranges often.
      println!("The number is {}", number);                 // Think of `number` as a variable name, this line prints the texts and numbers starting from 0 and ends in 2 excluding the number 3.
    }
  }

  fn main() {
    for number in 0..=5 {                                   // Adding the equals sign plus the number prints the text with the numbers inclusive.
      println!("The number prints {number}");               // This prints the command with the numbers and the number 5 inclusive.
    }
  }

  fn main() {
    for _ in 0..3 {                                         // The programmer can use the underscore if they don't need a variable name.
      println!("Printing same thing three times!");         // The code prints same line 3 times without the numbers.
    }
  }