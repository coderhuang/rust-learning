use std::io;

fn main() {
    println!("guess game!!!");
    println!("guess the rand num aha");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("fail to read line");
    println!("{}", guess);
}
