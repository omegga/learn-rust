#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

pub fn create_enum() {
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
