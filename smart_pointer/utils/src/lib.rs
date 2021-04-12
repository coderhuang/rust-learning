//! # 智能指针-工具create

pub use toby_smart_pointer::DropPointer;
pub use toby_smart_pointer::TobyBox;

#[cfg(test)]
mod tests {
    use crate::mock::{LimitTracker, Messenger};
    use std::cell::RefCell;

    #[test]
    fn refernce_explore() {
        // unimplemented!();
        let x = 5;
        let y = &x;

        println!("x:{}", x);
        println!("y:{}", y);
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

pub mod mock {

    pub trait Messenger {
        fn send(&self, message: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
}

pub mod toby_smart_pointer {

    use std::ops::Deref;
    use std::rc::Rc;
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

    pub struct DropPointer {
        x: String,
    }

    impl DropPointer {
        pub fn new(data: String) -> DropPointer {
            DropPointer { x: data }
        }
    }

    impl Drop for DropPointer {
        fn drop(&mut self) {
            println!("DropPointer data:{}", self.x);
        }
    }

    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    impl Drop for List {
        fn drop(&mut self) {
            println!("List Item leave it's scope");
        }
    }
}
