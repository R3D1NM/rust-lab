use std::fs::File;
use std::io::Error;
use std::io::Read;

fn read_username() -> Result<String, Error> {
    let file_result = File::open("hello.txt");

    // test if file_result is Ok or Err -> error propagation
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    // test if file.read_to_string(&mut username) is Ok or Err -> error propagation
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// ? operator
fn read_username_v2() -> Result<String, Error> {
    let mut file = File::open("hello.txt")?; // returns the value if Ok, returns Err if Err
    let mut username = String::new();
    file.read_to_string(&mut username)?; // returns the value if Ok, returns Err if Err
    Ok(username) // all Ok, return Ok(username)
}

// chaining ? operator
fn read_username_v3() -> Result<String, Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username) // all Ok, return Ok(username)
}

fn error_propagation() {
    let username = read_username();
    println!("{:?}", username);
}
