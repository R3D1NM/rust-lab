fn function_call() {
    let s = String::from("hello");
    string_length(s); // ownership of s is moved to string_length()
                      // println!("{}", s); // error[E0382]: use of moved value: `s`

    let x = 5;
    double(x); // x is copied to double()
    println!("{}", x); // 5
}

fn string_length(s: String) {
    println!("The length of '{}' is {}.", s, s.len());
}

fn double(x: i32) {
    println!("{} doubled is {}.", x, x * 2);
}
