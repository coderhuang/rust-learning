fn main() {
    println!("Hello, mod_test!");

    my_hosting::inner::call_print_one();
    my_hosting::call_print_two();

    let salad_breakfast = my_hosting::Breakfast::of(String::from("salad"));
    println!("salad_breakfast:{:#?}", salad_breakfast);

    let spring = my_hosting::Season::Spring;
    if let my_hosting::Season::Spring = spring {
        println!("spring is coming");
    }
}

mod my_hosting {

    pub mod inner {
        pub fn call_print_one() {
            super::print_one();
        }

        pub fn print_two() {
            println!("two");
        }
    }

    fn print_one() {
        println!("one");
    }

    pub fn call_print_two() {
        self::inner::print_two();
    }

    #[derive(Debug)]
    pub struct Breakfast {
        pub totast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn of(totast: String) -> Breakfast {
            Breakfast {
                totast: totast,
                season_fruit: String::from("apple"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }
}
