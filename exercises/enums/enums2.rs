// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
<<<<<<< HEAD
    // TODO: define the different variants used below
=======
>>>>>>> d1567fe030a6dac432df15cb675bbc7da1f3e5c3
    Move {
        x: i32,
        y: i32,
    },
    Echo(String),
    ChangeColor(u32, u32, u32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
