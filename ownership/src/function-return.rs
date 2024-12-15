fn function_return() {
    let s = String::from("hello");
    let s = string_length(s);
    println!("s is {}", s); // ownership of s is returned to the calling function
}

fn string_length(s: String) -> String {
    println!("The length of '{}' is {}.", s, s.len());
    s //return s: ownership of s is moved to the calling function
}
