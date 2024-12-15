fn functions() {
    println!("Hello, world!");
    a_function();
    print_number(5);

    println!("The sum of 5 and 10 is: {}", add_numbers(5, 10));

    let a = circle_area(3.0);
    println!("The area of a circle with radius 3 is: {}", a);
}

fn a_function() {
    println!("This is a function");
}

fn print_number(x: i32) {
    println!("The value of x is: {}", x);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    // return x + y;
    x + y // this is an expression
}
const PI: f64 = 3.141592;
fn circle_area(radius: f64) -> f64 {
    PI * radius * radius // no semicolon, this is an expression
}
