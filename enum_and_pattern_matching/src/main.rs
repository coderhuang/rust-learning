use std::io;

fn main() {
    println!("Hello, enum and pattern matching");

    let s = String::from("foo");
    assert_eq!("foo", s.as_str());

    println!("print which ip kind would you like to choose, 4 or 6?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("fail to read line");

    println!("input is：{}", input.as_str());
    let choosed_ip_kind = match input.as_str() {
        "4" => IpAddressKind::V4,
        "6" => IpAddressKind::V6,
        _ => IpAddressKind::V4,
    };

    let default_ip_address = create_default_ip_address(choosed_ip_kind);
    println!("{:?}", default_ip_address);

    let move_of_message = inner_mod::Message::Move { x: 1, y: 1 };
    move_of_message.print_self();
    let write_of_message = inner_mod::Message::Write(false, String::new());
    write_of_message.print_self();
    let change_color_of_message = inner_mod::Message::ChangeColor(0o7, 0b1000_1001, 0xafe1);
    change_color_of_message.print_self();
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

mod inner_mod {

    #[derive(Debug)]
    pub enum Message {
        Quit,
        Move { x: i32, y: u32 },
        Write(bool, String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        pub fn print_self(&self) {
            println!("enum Message value => {:#?}", self);
        }
    }
}
