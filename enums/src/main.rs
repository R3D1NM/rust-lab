#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct RGB(u8, u8, u8);

fn color_to_rgb(color: Color) -> RGB {
    // Match color to RGB
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
        // All variants must be handled
    }
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}

fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("Game started"),
        Message::WinPoint { who } => println!("Player {} won a point", who),
        Message::ChangePlayerName(name) => println!("Player name changed to {}", name),
        _ => println!("Unknown message"), // Handle all other cases (wildcard)
    }
}

fn main() {
    // Color enum
    let red: Color = Color::Red;
    let green: Color = Color::Green;

    println!("red: {:?}", red);

    // Compare two colors
    println!("red == green => {}", red == green);
    println!("red == Red => {}", red == Color::Red);

    // Message enum
    let m1 = Message::StartGame;
    let m2 = Message::WinPoint {
        who: "Alice".to_string(),
    };
    let m3 = Message::ChangePlayerName("Bob".to_string());

    // Convert color to RGB
    let rgb = color_to_rgb(Color::Green);
    println!("Green color in RGB: {:?}", rgb);

    // Handle message
    handle_message(&m1);
    handle_message(&m2);
    handle_message(&m3);

    // Option enum => used to handle null values
    let x = Some(5);
    let y = None;
    println!("Incremented x: {:?}", increment(x));
    println!("Incremented y: {:?}", increment(y));
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
