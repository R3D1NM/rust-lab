use std::fs::File;
use std::io::ErrorKind;
fn recoverable_error() {
    // recoverable error => Result<T, E>
    // enum Result<T, E> {Ok(T), Err(E)}

    let file = File::open("hello.txt"); // returns Result<T, E>
    match file {
        Ok(_) => {
            println!("File opened successfully");
        }
        Err(error) => {
            println!("Failed to open file: {:?}", error);
        }
    }

    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // error.kind() returns ErrorKind enum
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Failed to create file: {:?}", error),
            },
            _ => panic!("Failed to open file"),
        },
    };

    // unwrap() => returns the value if Ok, panics if Err
    let file: File = File::open("hello.txt").unwrap(); // panics if Err

    // expect() => returns the value if Ok, panics with the message if Err
    let file: File = File::open("hello.txt").expect("Failed to open file");

    // unrecoverable error => panic!
    // panic!("crash and burn");
}
