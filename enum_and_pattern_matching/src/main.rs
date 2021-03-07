use std::io;

fn main() {
    println!("Hello, enum and pattern matching");

    println!("print which ip kind would you like to choose");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("fail to read line");

    let choosed_ip_kind = if input == "V4" {
        IpAddressKind::V4;
    } else {
        IpAddressKind::V6;
    };

    let default_ip_address = create_default_ip_address(choosed_ip_kind);
    println!("{:?}", default_ip_address);
}

fn create_default_ip_address(ip_kind: IpAddressKind) -> IpAddress {
    match ip_kind {
        IpAddressKind::V4 => IpAddress::V4(0, 0, 0, 0),
        IpAddressKind::V6 => IpAddress::V6(String::from("aha")),
    }
}

enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}
