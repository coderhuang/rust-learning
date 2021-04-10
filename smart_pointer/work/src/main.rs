use utils::TobyBox;

fn main() {
    println!("Hello, smart pointer!");

    let x = "1piklsdsadhfgh";
    let y = TobyBox::new(&x);

    println!("x:{}", x);
    println!("y:{:#?}", *y);
}
