//! # 工具crate
//!
//! libs for util functions
//!

pub use rand_num::gen;
pub use string_operate::parse_input;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let gen = gen(100);
        assert!(gen > 0 && gen < 100);
    }

    #[test]
    fn parse_input_test() {
        let input = String::from("100 300");
        let (n1, n2) = parse_input(input);
        assert_eq!(n1, 100);
        assert_eq!(n2, 300);
    }
}

/// ## string操作的工具模块
pub mod string_operate {

    /// ### 解析输入
    ///
    /// 用于将字符串输入解析为两个数字
    /// 以空格为分隔符
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
}

pub mod rand_num {

    use rand::Rng;

    pub fn gen(max: i32) -> i32 {
        let x = rand::thread_rng().gen_range(1, max);
        return x;
    }
}
