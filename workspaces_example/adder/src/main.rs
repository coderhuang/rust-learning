use std::io;
use utils::{gen, parse_input};

fn main() {
    println!("Hello, workspaces !");

    println!("print two numers");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入有误!");

    let (x, y) = parse_input(input);
    println!("{}:{}", x, y);

    println!("rand by x:{}", gen(x));
    println!("rand by y:{}", gen(y));
}
