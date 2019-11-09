use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    
    if command == "123" {
        println!("Hi how are you");
    }
    else if command == "hello" {
        println!("Hello how are you");
    }
    else {
        println!("Hello");
    }
    println!("Args: {:?}", command);
}