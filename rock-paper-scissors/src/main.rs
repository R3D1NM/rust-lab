//packages
use std::io;

fn main() {
    println!("Pick your weapon: Rock, Paper, or Scissors"); //print in one line

    let mut decision = String::new(); //mutable variable

    //read line from the user
    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read line"); //error handling

    //trim the string
    let decision = decision.trim();

    //print the decision
    println!("You chose: {decision}");
}
