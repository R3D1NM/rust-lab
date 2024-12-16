#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

enum Message {
    StartGame,
    WinPoint { player: String },
    ChangePlayerName(String),
}

fn enum_basic() {
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
        player: "Alice".to_string(),
    };
    let m3 = Message::ChangePlayerName("Bob".to_string());
}
