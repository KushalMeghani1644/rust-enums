#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
struct IpAddress {
    kind: IpAddr,
    address: String,
}
impl IpAddress {
    fn display(&self) {
        match &self.kind {
            IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("IPv6: {}", addr),
        }
    }
}
fn main() {
    let home = IpAddress {
        kind: IpAddr::V4(127, 0, 0, 1),
        address: String::from("127,0,0,1"),
    };
    let loopback = IpAddress {
        kind: IpAddr::V6(String::from("::1")),
        address: String::from("::1"),
    };
    home.display();
    loopback.display();
}
