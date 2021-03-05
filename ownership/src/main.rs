fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    takes_ownership(s);
    let s = gives_ownership();
    println!("{}", s);
    let s = takes_and_gives_back(s);
    println!("{}", s);

    let mut s = String::from("what the");
    mut_reference(&mut s);

    let (s, len) = multiple_return_value(s);
    println!("s:{};len:{}", s, len);
    
    println!("---------------------寂寞分割线---------------------");

    let len = calculate_length(&s);
    println!("s:{};len:{}", s, len);
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn multiple_return_value(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn mut_reference(s: &mut String) {
    (*s).push_str(" world!");
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn gives_ownership() -> String {
    let s = String::from("啊哈");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}
