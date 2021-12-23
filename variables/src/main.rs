const MAX_DIMENSION: u32 = 1 << 6;

fn main() {
    println!("{}", MAX_DIMENSION);

    let tup = (100, 'a', 1.3);
    let (x, y, z) = tup;
    println!("x:{};\ny:{};\nz:{}", x, y, z);
    println!("------------神秘分割线----------");
    println!("x:{};\ny:{};\nz:{}", tup.0, tup.1, tup.2);
    
    let i = incre_one(1);
    println!("call incre_one() with argument 1; return => {}", i);
    
    for number in (1..4).rev() {
        println!("{}", number);
    }
    
    println!("------------神秘分割线----------");
    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for ele in array.iter() {
        println!("{}", ele);
    }

    for (i, ele) in array.iter().enumerate() {
        println!("{}:{}", i, ele);
    }

    print_fibonacci(5);

    shadowing();
}

fn incre_one(i: i32) -> i32 {
    i + 1
}

fn print_fibonacci(u: u32) {
    let mut x = 0;
    let mut y = 1;
    let mut n = 0;

    while n < u {
        let m = y;
        y = x + y;
        x = m;
        println!("x=>{};y=>{}", x, y);
        n += 1;
    }
}

fn shadowing(){

    let x = 10;
    println!("x:{}",x);
    
    let x = x+1;
    println!("x:{}",x);
    
    let x = x * 2;
    println!("x:{}",x);
    
    let x = "打完收工😁";
    println!("x:{}",x);
    
    let x = false;
    println!("x:{}",x);
    
    let x = b'0';
    println!("x:{}",x);

    let x:u32 = 0xffeedcba;
    println!("x:{}",x);
    
    let x:u32 = 0o01234560123;
    println!("x:{}",x);
    
    let x:u32 = 0b0111_100;
    println!("x:{}",x);
    
}
