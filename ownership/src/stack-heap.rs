fn stack_heap() {
    //allocate memory on the heap
    let mut s: String = String::from("hello"); // string from literal "hello"
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // hello

    {
        // stack
        let x = 3;
        let y = x; // all primitive types implement the Copy trait
        println!("x: {}, y: {}", x, y); // x: 3, y: 3

        // heap
        let s1 = String::from("hello"); // located on the heap
        println!("s1: {}", s1);
        let s2 = s1; // ownership of s1 is moved to s2
                     // println!("{}", s1); // error[E0382]: use of moved value: `s1`
        println!("s2: {}", s2);

        let s3 = s2.clone(); // deep copy
        println!("s2: {}, s3: {}", s2, s3); // s2: hello, s3: hello
    }
}
