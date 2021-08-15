#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let home2 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback2 = IpAddrEnum::V6(String::from("::1"));

    println!("{:#?}", home2);
    println!("{:#?}", loopback2);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}