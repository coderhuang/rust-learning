use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, exception handle!");

    // panic!("啊哈");

    let open_result = File::open("啊哈.txt");
    let f =  match open_result {
        Ok(file) => file,
        Err(excep) => match excep.kind() {
            ErrorKind::NotFound => match File::create("啊哈.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    println!("{}", f.metadata().is_ok());
}
