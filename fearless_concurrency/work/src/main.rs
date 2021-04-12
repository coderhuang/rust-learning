use std::{error::Error, result::Result, sync::mpsc, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, 无惧并发!");

    let v = vec!["123", "lol", "pop", "push", "s"];
    let handle = thread::spawn(move || {
        for (i, val) in v.iter().enumerate() {
            println!("{}:{}", i, val);
            thread::sleep(Duration::from_secs(1))
        }
    });
    // thread::spawn(move || {
    //     for (i, val) in v.iter().enumerate() {
    //         println!("{}:{}", i, val);
    //         thread::sleep(Duration::from_secs(1))
    //     }
    // });

    handle.join().unwrap();
    for i in 1..5 {
        println!("num {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    let (tx, rx) = mpsc::channel();
    let tx_handler = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // tx_handler.join().unwrap();
    let x = rx.recv()?;
    println!("receive message:{}", x);

    Ok(())
}
