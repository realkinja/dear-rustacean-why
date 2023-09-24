use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::io::{self, Write};

fn inputprompt(prompt: &str){
    print!("{prompt}");
    let _ = io::stdout().flush();
}

fn main(){
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    inputprompt("Enter first number: ");
    io::stdin()
        .read_line(&mut input1)
        .expect("no number!!!");

    let handleint1 = thread::spawn(move || {
        let val: i32;
        match input1.trim().parse::<i32>() {
            Ok(_value) => {
                val = input1.trim().parse::<i32>().unwrap();
                tx1.send(val).unwrap();
            }
            Err(_) => {
                println!("Not an integer!");
                val = 0;
                tx1.send(val).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(2));
    });

    handleint1.join().unwrap();
    let int1 = rx1.recv().unwrap();

    inputprompt("Enter second number: ");
    io::stdin()
        .read_line(&mut input2)
        .expect("no number!!!");

    let handleint2 = thread::spawn(move || {
        let val: i32;
        match input2.trim().parse::<i32>() {
            Ok(_value) => {
                val = input2.trim().parse().unwrap();
                tx2.send(val).unwrap();
            }
            Err(_) => {
                println!("Not an integer!");
                val = 0;
                tx2.send(val).unwrap();
            }
        };
        thread::sleep(Duration::from_millis(2));
    });

    handleint2.join().unwrap();
    let int2 = rx2.recv().unwrap();

    println!("The sum of int1 and int2 is: {}", int1 + int2);
}