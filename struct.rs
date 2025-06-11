//This file contains code snippets and examples of structs in Rust.

fn main() {
  let mut new_cat = Animal::new_cat();
  //new_cat.check_type();
  let mut new_dog = Animal::new_dog();
  }
#[derive(Debug)]
enum AnimalType {
    Dog,
    Cat
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn new_dog() -> Self {
        Self {
            age: 15,
            animal_type: AnimalType::Dog,
        }
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Cat => println!("The animal is a {self:?}"),
            AnimalType::Dog => println!("The animal is a {self:?}"),
        }
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        self.age = 15;
        println!("The animal is now changed to a {self:?}");
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        self.age = 10;
        println!("Animal is now changed to a {self:?}");
    }

    fn print_age(&mut self) {
       match self.age {
        10 => println!("This is obviously a Cat!"),
        15 => println!("Thats a Dog right there!"),
        _ => println!("The animals are underaged"),
       }
    }

}