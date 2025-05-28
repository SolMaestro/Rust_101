//This file contains examples and code snippets about Rust collections and compound types.

//Arrays
let array1 = ["Rust", "Collections", "Compound", "Types"]; //In Rust, Arrays are created by just putting data inside square brackets seperated by commas. Arrays have intresting types, this array is of type [&str,4].
let array2 = [500, 200, 150, 300, 200]; //Arrays in Rust are considered rigid because they must only contain same type and cannot change their size. This array is of type [i32,5].
let array3 = ["Rust"; 10]; //The programmer can also create an array this way and when printed using the macro `println!` it prints `Rust` 10 times. Yes, this is also of type [&str,10]. This method is used to create byte buffers (an array of bytes).
println!("My first Array contains {:?}", array1); //Notice the `:?` in the curly brackets? that's called Display printing. Like the array and other compound types, Some variables won't be able to print with {} and the programmer needs Debug printing.
println!("My second Array contains {}", array2); //If the programmer tries to print the whole content of an array without the display printing`:?`, the compiler prints an error message reminding the programmer that the variable does not implement Display format and cannot be formatted with the default formatter.
println!("The third content of my array2 is {}" array2[2]); //printing a particular index in an array does not require using the display printing format. To get the index we put the number of the index in square brackets close to the array name. This line prints `The third content of my array2 is 150`.
//PS: As with most programming languages, indexs starts at 0 and Rust programming language is no different.

//Looping through Arrays
