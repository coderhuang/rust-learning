fn main() {
    println!("struct");

    let default_province = String::from("未知之地");
    let default_city = String::from("未知之地");
    let default_street = String::from("未知之地");

    let default_address = Address {
        province: default_province,
        city: default_city,
        street: default_street,
        no: 1,
        valid_within_the_last_month: false,
    };

    println!(
        "default_address=>{};{};{};{},{}",
        default_address.province,
        default_address.city,
        default_address.street,
        default_address.no,
        default_address.valid_within_the_last_month
    );

    let address_one = Address {
        province: String::from("遥远的地方"),
        city: String::from("有一座山"),
        street: String::from("有一群蓝精灵"),
        ..default_address
    };

    println!(
        "address_one=>{};{};{};{},{}",
        address_one.province,
        address_one.city,
        address_one.street,
        address_one.no,
        address_one.valid_within_the_last_month
    );

    let black = MixVector(0, 1, 02.0);
    let origin = Point(0, 1, 2);

    println!("{};{}", black.2, origin.1);

    let rec = (10, 10);
    println!("The area of the rectangle is {} square pixels.", area(rec));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{:?}", rect1);
    println!("{:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let hold_flag = rect2.can_hold(&rect3);
    println!("hold_flag:{}", hold_flag);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

struct MixVector(i32, i64, f32);
struct Point(i32, i32, i32);

struct Address {
    province: String,
    city: String,
    no: i32,
    street: String,
    valid_within_the_last_month: bool,
}
