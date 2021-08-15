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

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

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

    // let m = Message::Write(String::from("hello"));
    // m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}