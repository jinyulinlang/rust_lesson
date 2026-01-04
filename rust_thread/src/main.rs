use std::thread;
use std::time::Duration;

fn main() {
    let hadler = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Hello, world!");
    hadler.join().unwrap(); 
}
