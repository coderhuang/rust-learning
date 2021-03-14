extern crate collection_lib_example;

use collection_lib_example::string_funs;

fn main() {
    println!("Hello, collection!");

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let three = &v[2];
    println!("value of index 2 at v:{}", three);

    match v.get(1) {
        Some(two) => println!("value of index 1 at v:{}", two),
        None => println!("there is nothing"),
    }

    let v = vec![100, 0o100, 0x12a, 0b1111_1111];
    for value in &v {
        println!("{}", value);
    }

    let formated_string = string_funs::formate(
        "在山的那边".to_string(),
        "海的那边".to_string(),
        "有一群蓝精灵".to_string(),
    );
    println!("{}", formated_string);
}
