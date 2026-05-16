#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    // 居然是特殊构造函数吗
    Resize{width:i32,height:i32},
    Move(Point),
    Echo(String),
    ChangeColor(u8,u8,u8),
    Quit
}

/**
 * 这些变体不是函数，是构造函数。每次写 Message::Move(Point{x:10,y:15}) 就是在创建（构造）一个 Message 类型的值。
 * 用处在于：同一种类型表达不同形态的数据，然后用 match 分流处理。
 * 核心价值：把不同的"消息种类"统一成一个类型，一个 Vec<Message> 就能装下各种形态的消息，match 强制你处理每一种——如果漏了一个变体，编译器会报错。
 * 这其实就是 Rust 版的多态/代数数据类型——不用继承，不用接口，直接靠枚举和模式匹配搞定
 */

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
