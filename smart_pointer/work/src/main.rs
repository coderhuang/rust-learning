use std::mem::drop;
use std::rc::Rc;
use utils::toby_smart_pointer::List::{Cons, Nil};
use utils::DropPointer;
use utils::TobyBox;

fn main() {
    println!("Hello, smart pointer!");

    let x = "1piklsdsadhfgh";
    let y = TobyBox::new(&x);

    println!("x:{}", x);
    println!("y:{:#?}", *y);
    println!("y:{:#?}", y);

    let c = DropPointer::new(String::from("my stuff"));
    let d = DropPointer::new(String::from("other stuff"));
    println!("DropPointers created.");
    drop(c);
    drop(d);

    let a = Rc::new(Cons(
        11,
        Rc::new(Cons(50, Rc::new(Cons(100, Rc::new(Nil))))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
