fn borrow() {
    let s = String::from("hello");
    let (len, s) = string_length(s);
    println!("The length of '{}' is {}.", s, len);

    // borrow
    let s = String::from("hello");
    let len = string_length_borrow(&s); // borrow the ownership of s
                                        // &s: s is borrowed, so it can be used after the function call
                                        // References are immutable by default
    println!("The length of '{}' is {}.", s, len);

    // mutable borrow
    let mut s = String::from("hello");
    string_length_mut_borrow(&mut s); // borrow the ownership of s
                                      // &mut s: s is borrowed mutably, so it can be modified after the function call
                                      // if s is borrowed mutably, no other borrow can occur => prevent data races
    println!("{}", s);
}

fn string_length(s: String) -> (usize, String) {
    println!("The length of '{}' is {}.", s, s.len());
    (s.len(), s)
}

fn string_length_borrow(s: &String) -> usize {
    s.len()
}

fn string_length_mut_borrow(s: &mut String) {
    s.push_str(", world!");
    println!("{}", s);
}
