#[derive(Debug)]
enum Message {
    Write(String),
}

// Like structs, we can define methods on enums
impl Message {
    fn call(&self) {}
}

pub fn enum_method() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
