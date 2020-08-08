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

pub fn with_a_struct() {
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
