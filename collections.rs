// This file contains examples and code snippets about Rust collections and compound types.

// Arrays
let array1 = ["Rust", "Collections", "Compound", "Types"];            // In Rust, Arrays are created by just putting data inside square brackets seperated by commas. Arrays have intresting types, this array is of type [&str,4].
let array2 = [500, 200, 150, 300, 200];                               // Arrays in Rust are considered rigid because they must only contain same type and cannot change their size. This array is of type [i32,5].
let array3 = ["Rust"; 10];                                           // The programmer can also create an array this way and when printed using the macro `println!` it prints `Rust` 10 times. Yes, this is also of type [&str,10]. This method is used to create byte buffers (an array of u8).
println!("My first Array contains {:?}", array1);                    // Notice the `:?` in the curly brackets? that's called Display printing. Like the array and other compound types, Some variables won't be able to print with {} and the programmer needs Debug printing.
println!("My second Array contains {}", array2);                     // If the programmer tries to print the whole content of an array without the display printing`:?`, the compiler prints an error message reminding the programmer that the variable does not implement Display format and cannot be formatted with the default formatter.
println!("The third content of my array2 is {}" array2[2]);          // Printing a particular index in an array does not require using the display printing format. To get the index we put the number of the index in square brackets close to the array name. This line prints `The third content of my array2 is 150`.
                                                                     // PS: As with most programming languages, indexs starts at 0 and Rust programming language is no different.

// Looping through Arrays

for numbers in array1 {                                             // The programmer can loop through Arrays by using the for loop. the number in this expression acts as a variable name that is used to print the individual contents of the Array. 
  println!("Numbers in my array include {}", numbers);              // This line prints the whole content of the array without using the display/debug format.
}

let slice1 = &array1[..3];                                         // The programmer can also get slices of an array by using a reference to that array and representing the index in square brackets. the dots signify range as this simply means to print from index 0 to but not including the third item in the array.
let slice2 = &array2[1..=4];                                      // This range does not include the first item in the array as indexes in Rust starts with 0 but it prints the last item inlusive. this can be seen by the equal sign representation.
let slice3 = &array3[..];                                         // Without specifying the index, this range prints the whole item in the array.
println!("The items in my Array are {:?}", slice1);               // Printing the items in an array using the slice method requires the programmer to use the display/debug format else the compiler will print an error message.

// Vectors
// A vector also pronounced as vec which ryhmes with deck is also used to store more than one value in a data structure.
let name1 = String::from("I Love Rust");
let name2 = String::from("Programming is cool");

let mut vec: Vec<String> = Vec::new();          // This line declares a new vector with an explicitly declared type. Without the explicit type annotation, Rust can still infer the type when the programmer starts pushing data into the vector.
vec.push(name1);                                // The push method pushes some data into the vector and from here Rust infers the type of vector if we had not declared it explicitly.
vec.push(name2);                               // This line pushes a second data into the vector. Remember that this data and every other one going into the vector must be of a string type else when the code compiles, the compiler will print an error message.

let vec2 = vec![200, 4000, 800, 55, 65, 30];   // For conveninence, the programmer can also create a vector using this approach.
vec2.push(26);                                 // The programmer can also push data into a Vec created with the macro vec!. This is valid code, as long as same data type is being pushed into the Vec.

let vec_slice = &vec2[..];                    // Just like Arrays, the programmer can get slices of a Vec by using the reference to the vec's variable and inputting the required index inside square brackets. This line tells Rust to get the whole values in the Vec.
let vec_slice2 = &vec[1..3];                  // This line tells Rust to get the values at index 1 to without including 3.
let vec_slice3 = &vec[..=5];                  // Lastly, this line requires Rust to get the values from 0 index  to and including 5. P.S: using this approach returns an &i32.
let get_vec = vec2.get(5);                    // The programmer can also get a value in a Vec by using the get method. This returns a type option<&i32>.

for i in vec2 {                               // Like Arrays, the programmer can loop through the vectors using the for loop.
  println!("{}", i);                          // This line prints all the values in the vector without needing the Debug format.
}                                             // P.S: If the programmer is using this code in the same scope with a for loop that changes the contents of the vectors then the vec variable next to in will have a reference `&`.

for x in &mut vec2 {                          // This line iterates over a vector and makes changes to the values in the vector by adding 50 each value. it takes a mutable reference of the Vec.
*x += 50;                                     // To get to the values the mutable reference points to in the vector, the programmer uses a deference operator `*` to access the value in x before adding the values with `+=`.
println!("My vector values are now {}", x);   // This line prints the values in the Vec without using the debug format.
}