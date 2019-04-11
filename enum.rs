#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(msg) => println!("message: {}", msg),
            Message::ChangeColor(r,g,b) => println!("Changing color to: R{}G{}B{}", r,g,b),
            _ => println!("not a Message")
        }
        // method body would be defined here
        //println!("{:#?}",self);
    }
}

fn main() {
    //println!("Hello, world!");
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(1,2,3);
    w.call();
    c.call();
}

/* https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=abe5643cbd7f10c34d2d32e1a11d5a24 */
