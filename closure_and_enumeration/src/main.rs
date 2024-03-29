//! # 文档
//!
//! ```
//! fn(){
//!     println!("这是一段代码");
//! }
//!
//! ```

use inner_utils::*;
use std::io;

fn main() {
    println!("Hello, closure!");

    let mut cacher = Cacher::new(|a| a);
    let cached_val = cacher.val(3000);
    println!("cached val:{}", cached_val);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("equal_to_x(y):{}", equal_to_x(y));

    fn equals(x: i32, y: i32) -> bool {
        x == y
    }

    let equals_flag = equals(100, 132);
    println!("equals_flag:{}", equals_flag);

    let mut input = String::new();
    println!("please input two numbers used for compare");
    io::stdin()
        .read_line(&mut input)
        .expect("io读取用户输入错误");

    let (n1, n2) = parse_input(input);
    println!("{}", equals(n1, n2));
}

/// # markdown example for inner_utils module
///
/// ```
/// fn(){
///    println!("这是一段代码");    
/// }
/// ```
mod inner_utils {

    /// ### 解析输入
    /// 用于将字符串输入解析为两个数字
    /// 以空格为分隔符
    ///
    /// ```
    /// let input = "100 100";
    /// let (n1, n2) = parse_input(input);
    /// println!("{}", equals(n1, n2));
    ///
    /// ```
    ///
    ///
    pub fn parse_input(input: String) -> (i32, i32) {
        let len = input.len();
        let mut split_index = 0;
        for (i, c) in input.chars().enumerate() {
            if c == ' ' {
                split_index = i;
                break;
            }
        }
        let num1_str = &input[0..split_index];
        let num2_str = &input[(split_index + 1)..len];
        let num1 = num1_str.trim().parse().unwrap();
        let num2 = num2_str.trim().parse().unwrap();
        (num1, num2)
    }

    pub fn my_panic(message: String) {
        panic!(message);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > 6 {
            return Some(self.count);
        } else {
            return None;
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    val: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            val: None,
        }
    }

    fn val(&mut self, val: u32) -> u32 {
        match self.val {
            Some(v) => {
                if v == val {
                    return v;
                }
                let value = (self.calculation)(val);
                self.val = Some(value);
                value
            }
            None => {
                let value = (self.calculation)(val);
                self.val = Some(value);
                value
            }
        }
    }
}
