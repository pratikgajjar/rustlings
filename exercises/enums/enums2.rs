// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,
    Echo { msg: String },
    Move { x: i32, y: i32 },
    ChangeColor {r: i32, g: i32, b: i32 }
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }

    fn Echo(&self, msg: &str) {
        // println!("{:?}", msg);
    }

    fn ChangeColor(&self, r :i32, g :i32, b :i32) {
        // println!("{:?}", r, g, b);
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
