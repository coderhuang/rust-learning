use std::io;

fn main() {
    println!("Hello, grep!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    split_input_by_white_space(&input);
}

fn split_input_by_white_space<'a>(wait_to_split: &'a String) -> Vec<&'a str> {
    let mut start = 0;
    let len = wait_to_split.len();

    let mut v = Vec::new();
    for (i, c) in wait_to_split.chars().enumerate() {
        if c == ' ' {
            v.push(&wait_to_split[start..i]);
            start = i + 1;
        }
    }
    v.push(&wait_to_split[start..(len - 1)]);

    let file_path = &v[1];
    let search_str = &v[2];
    println!("{}:{}", file_path, search_str);
    v
}
