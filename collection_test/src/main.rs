extern crate collection_lib_example;

mod hash_map_test;

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
    for (i, val) in v.iter().enumerate() {
        println!("v at {} value :{}", i, val);
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
    for c in formated_string.chars() {
        print!("{}", c);
    }
    println!();
    for b in formated_string.bytes() {
        println!("{}", b);
    }
    hash_map_test::new();
    hash_map_test::lets_panic();
    hash_map_test::get_val("blue");
    hash_map_test::insert_test("red");
    hash_map_test::insert_test_1("green");
    hash_map_test::insert_test_2();
}
