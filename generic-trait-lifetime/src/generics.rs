fn smallest_i32(list: &[i32]) -> &i32 {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest_char(list: &[char]) -> &char {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

// generic function
// T: generic type, std::cmp::PartialOrd: trait bound
fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

// generic struct
struct Point<T> {
    x: T,
    y: T,
}

// generic struct with multiple types
struct Point2<T, U> {
    x: T,
    y: U,
}

// generic enum
enum Option<T> {
    Some(T),
    None,
}

// generic method
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn generics() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = smallest_i32(&numbers);
    println!("The smallest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = smallest_char(&chars);
    println!("The smallest char is {}", result);

    // using generic function
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    let result = smallest(&numbers);
    println!("The smallest number is {}", result);

    let result = smallest(&chars);
    println!("The smallest char is {}", result);

    // using generic struct
    let p1 = Point { x: 5, y: 10 }; // Point<i32>
    let p2 = Point { x: 1.5, y: 4.7 }; // Point<f64>
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
}
