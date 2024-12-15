fn main() {
    //slice
    let s = String::from("hello world");
    let hello: &str = &s[0..5]; // &s[0..5] is a slice that references a portion of the String
    println!("hello: {}", hello);
    let world: &str = &s[6..];
    println!("world: {}", world);
    let all: &str = &s[..];
    println!("all: {}", all);

    let word = first_word(&s);
    println!("word: {}", word);

    //string literals are slices
    let s: &str = "hello world";
    let hello = String::from("hello world");
    let word1 = first_word_literal(s);
    let word2 = first_word_literal(&hello); // &String -> &str
    println!("hello: {}, word1: {}, word2: {}", hello, word1, word2);

    //array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // &[i32]
    println!("slice: {:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return a slice that starts at the beginning of the string and ends at the space
        }
    }
    &s[..]
}

fn first_word_literal(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return a slice that starts at the beginning of the string and ends at the space
        }
    }
    &s[..]
}
