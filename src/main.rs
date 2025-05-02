#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl message {
    fn write(&self) {
        match self {
            message::Quit => println!("Quit"),
            message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            message::Write(text) => println!("Write: {}", text),
        }
    }
}
struct IpAddress {
    kind: IpAddr,
    address: String,
}
fn main() {
    let ipaddress = IpAddress {
        kind: (IpAddr::V4(127, 0, 0, 1)),
        address: String::from("IpV4"),
    };
    println!("IpAddress: {:?}", ipaddress.kind);
}
