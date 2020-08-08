pub fn ex_1() {
    // Instead of using a struct to associate a value to an enum,
    // we can directly associate a value to the enum variants
    #[derive(Debug)]
    enum IpAddKind {
        V4(String),
        V6(String),
    }
    let home = IpAddKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddKind::V6(String::from("::1"));
    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);
}

pub fn ex_2() {
    // Each variant can have different types and amounts of associated data
    #[derive(Debug)]
    enum IpAddKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddKind::V4(127, 0, 0, 1);
    let loopback = IpAddKind::V6(String::from("::1"));
    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);
}

pub fn ex_3() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let message = Message::Quit;
    let message_2 = Message::Move { x: 1, y: 2 };
    let message_3 = Message::Write(String::from("hello"));
    let message_4 = Message::ChangeColor(0, 0, 0);
    println!("message = {:?}", message);
    println!("message_2 = {:?}", message_2);
    println!("message_3 = {:?}", message_3);
    println!("message_4 = {:?}", message_4);
}
