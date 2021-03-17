use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, exception handle!");

    // panic!("啊哈");

    let open_result = File::open("啊哈.txt");
    let f = match open_result {
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

    let f = File::open("123.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("123.txt").unwrap_or_else(|ee| {
                panic!("打开和创建都失败:{:#?}", ee);
            })
        } else {
            panic!("Problem opening the file: {:?}", e);
        }
    });
    println!("{}", f.metadata().is_ok());

    let open_result_1 = File::open("123.txt");
    if let Ok(_) = open_result_1 {
        println!("let ok is done");
    }

    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("句柄");
    let read_string = lol::read_string_from_file("啊哈.txt");
    if let Ok(s) = read_string {
        println!("{}", s);
    }
}

mod lol {

    use std::io;
    use std::fs::File;
    use std::io::Read;
    
    pub fn read_string_from_file(path: &str) -> Result<String, io::Error> {
        let f = File::open(path);

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(size) => {
                println!("{}", size);
                Ok(s)
            },
            Err(e) => Err(e),
        }
    }
}
