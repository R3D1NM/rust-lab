fn main() {
    let x = 4;

    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 3 == 1 {
        println!("x divided by 3 leaves a remainder of 1");
    } else {
        println!("x divided by 3 leaves a remainder of 2");
    }

    // if is an expression
    let condition = true;
    let y = if condition { 5 } else { 6 };
    println!("The value of y is: {}", y);

    // infinite loop
    let mut counter = 0;
    loop {
        println!("again!");
        counter += 1;
        if counter == 3 {
            break;
        }
    }

    // loop with return value
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }
    println!("DONE!");

    // for loop
    for element in a {
        println!("the value is: {}", element);
    }
    println!("DONE!");

    // for loop with range [0, 5)
    for i in 0..5 {
        println!("i = {}", i);
    }
    println!("DONE!");

    // for loop with range reverse [5, 0)
    for i in (0..5).rev() {
        println!("i = {}", i);
    }
}
