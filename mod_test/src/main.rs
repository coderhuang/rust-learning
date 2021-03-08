fn main() {
    println!("Hello, mod_test!");

    my_hosting::inner::call_print_one();
    my_hosting::call_print_two();
}



mod my_hosting{

    pub mod inner{
        pub fn call_print_one(){
            super::print_one();
        }

        pub fn print_two(){
            println!("two");
        }
    }

    fn print_one(){
        println!("one");
    }

    pub fn call_print_two(){
        self::inner::print_two();
    }
}