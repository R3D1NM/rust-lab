struct User {
    name: String,
    email: String,
    active: bool,
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
    }
}

fn struct_basic() {
    println!("Hello, world!");

    // Create a new user instance
    let mut user = User {
        name: String::from("John Doe"),
        email: String::from("johnDoe@email.com"),
        active: true,
    };

    // Assign a new value to the mutable user instance
    user.email = String::from("john@gmail.com");

    // Access the user fields
    println!("User name: {}", user.name);
    println!("User email: {}", user.email);

    // Create a new user instance using a function
    let u = build_user(String::from("Jane Doe"), String::from("jane@email.com"));
    println!("User name: {}", u.name);

    // Create a new user instance using struct update syntax
    let u2 = User {
        active: false,
        ..user
    };
    println!("User name: {}", u2.name);

    // Create a tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let point = Point(1, 2, 3);

    println!("Black color: {}, {}, {}", black.0, black.1, black.2);
    println!("Point: {}, {}, {}", point.0, point.1, point.2);
}
