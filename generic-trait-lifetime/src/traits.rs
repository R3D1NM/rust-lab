use std::fmt::Debug;
use std::fmt::Display;

trait Greet {
    fn greeting(&self) -> String;
}

#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
    Tiger,
}

// Implement the Greet trait for the Pet enum
impl Greet for Pet {
    fn greeting(&self) -> String {
        match self {
            Pet::Dog => "Woof".to_string(),
            Pet::Cat => "Meow".to_string(),
            Pet::Tiger => "Roar".to_string(),
        }
    }
}

struct Person {
    name: String,
    age: u8,
}

impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("Hello!")
    }
}

// both one and another are of type Greet
// the meet function can accept any type that implements the Greet trait
fn meet(one: &impl Greet, another: &impl Greet) {
    println!("one says: {}", one.greeting());
    println!("another says: {}", another.greeting());
}

// both one and another are of type T
// the pet_meet function can accept any type that implements the Greet trait but with same type
fn pet_meet<T: Greet>(one: &T, another: &T) {
    println!("one says: {}", one.greeting());
    println!("another says: {}", another.greeting());
}

// both one and another are of type T
// the meet_with_debug function can accept any type that implements both the Greet trait and Debug trait
fn meet_with_debug<T: Greet + Debug>(one: &T, another: &T) {
    println!("{:?} says: {}", one, one.greeting());
    println!("{:?} says: {}", another, another.greeting());
}

// multiple traits
// both one and another are of type T
// the meet_with_debug_and_display function can accept any type that implements both the Greet trait and Debug trait
fn meet_with_debug_and_display<T: Greet + Debug + Display>(one: &T, another: &T) {
    println!("{:?} says: {}", one, one.greeting());
    println!("{:?} says: {}", another, another.greeting());
}

// where clause
// both one and another are of type T
// the meet_with_debug_and_display_with_where function can accept any type that implements both the Greet trait and Debug trait
fn meet_with_debug_and_display_with_where<T>(one: &T, another: &T)
where
    T: Greet + Debug + Display,
{
    println!("{:?} says: {}", one, one.greeting());
    println!("{:?} says: {}", another, another.greeting());
}

fn traits() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let dog = Pet::Dog;
    let cat = Pet::Cat;
    let tiger = Pet::Tiger;

    meet(&person, &dog);
    pet_meet(&dog, &cat);
    meet_with_debug(&cat, &tiger);
}
