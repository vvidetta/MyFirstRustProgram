use std::io;

fn main() {
    println!("What is your name?");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let name = buffer;
    let mut buffer = String::new();
    println!("How old are you?");
    io::stdin().read_line(&mut buffer);
    let age = buffer.parse::<i32>().unwrap();
    println!("Hello, {}. You are {} years old!", name, age);
}
