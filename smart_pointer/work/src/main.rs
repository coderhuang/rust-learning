use utils::DropPointer;
use utils::TobyBox;

fn main() {
    println!("Hello, smart pointer!");

    let x = "1piklsdsadhfgh";
    let y = TobyBox::new(&x);

    println!("x:{}", x);
    println!("y:{:#?}", *y);

    let c = DropPointer::new(String::from("my stuff"));
    let d = DropPointer::new(String::from("other stuff"));
    println!("DropPointers created.");
}
