// variable shadowing
fn variables() {
    let x = 3;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    {
        let x = x * 2;
        println!("The value of x from the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
