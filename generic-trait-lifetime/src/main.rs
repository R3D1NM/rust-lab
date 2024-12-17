fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("one");

    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);
}

// The function signature tells Rust that the lifetime of the reference returned by the function
// 'a is the same as the lifetime of the references passed in.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // The lifetime of ImportantExcerpt is the same as the lifetime of first_sentence.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    // If the method is called with self, the returned reference will be valid as long as the object is valid.
    // All the annotations in this method is unnecessary because the lifetime of self is the same as the lifetime of the object.
    fn notice<'b>(&self, text: &'b str) -> &'a str {
        println!("ImportantExcerpt: {}", text);
        self.part
    }
}

// static lifetime
// The lifetime of the reference is the entire duration of the program.
// All string literals have the 'static lifetime.
fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}
