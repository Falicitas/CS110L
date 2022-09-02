struct Q {
    data: String,
}
enum Message {
    Quit,                       //无类型，不包含任何值
    Move { x: i32, y: i32 },    //包含匿名结构体
    Write(String),              //包含一个字符串
    ChangeColor(u32, u32, u32), //包含三元组
}

// #[derive(Debug)]
impl Message {
    fn print(&self) {
        match self {
            Message::Quit => (),
            Message::ChangeColor(x, y, z) => (),
            Message::Move { x, y } => println!("{} {}", x, y),
            _ => (),
        }
    }
}
fn main() {
    let x = Message::Quit;
    let y = Message::Move { x: 1, y: 2 };
}
