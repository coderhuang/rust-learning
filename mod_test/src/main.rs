fn main() {
    println!("Hello, mod_test!");

    my_hosting::inner::call_print_one();
}



mod my_hosting{

    pub mod inner{
        pub fn call_print_one(){
            super::print_one();
        }
    }

    fn print_one(){
        println!("one");
    }
}