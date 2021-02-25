use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess game!!!");
    println!("guess the rand num aha");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
