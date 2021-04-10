//! # 智能指针-工具create

pub use toby_smart_pointer::TobyBox;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn refernce_explore() {
        // unimplemented!();
        let x = 5;
        let y = &x;

        println!("x:{}", x);
        println!("y:{}", y);
    }
}

pub mod toby_smart_pointer {

    use std::ops::Deref;
    pub struct TobyBox<T>(T);

    impl<T> TobyBox<T> {
        pub fn new(x: T) -> TobyBox<T> {
            TobyBox(x)
        }
    }

    impl<T> Deref for TobyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
}
