#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_example() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    // Tuple
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect)
    );

    // Rectangle struct
    let r = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&r)
    );

    // Debug print
    println!("Rectangle: {:?}", r);
    dbg!(r); // debug macro
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
