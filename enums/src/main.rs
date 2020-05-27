fn main() {
    create_enum();
    with_a_struct();
    enum_associated_value();
    enum_associated_value_2();
    enum_associated_value_3();
    enum_method();
}

fn create_enum() {
    #[derive(Debug)]
    enum IpAddKind {
        V4,
        V6,
    }
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;
    // both values are of the same type: IpAddrKind
    // We can then define and call a function that takes any IpAddrKind
    fn route(ip_kind: IpAddKind) {
        println!("ip_kind = {:?}", ip_kind);
    }
    route(IpAddKind::V4);
    route(four);
    route(six);
}

fn with_a_struct() {
    #[derive(Debug)]
    enum IpAddKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };
    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);
}

fn enum_associated_value() {
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

fn enum_associated_value_2() {
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

fn enum_associated_value_3() {
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

fn enum_method() {
    #[derive(Debug)]
    enum Message {
        Write(String),
    }

    // Like structs, we can define methods on enums
    impl Message {
        fn call(&self) {}
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}
