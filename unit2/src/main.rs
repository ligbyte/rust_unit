
use std::io;
fn main() {
    let mut input = String::new();
     
    println!("Hello, world!");
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Your name is {}", input.trim()),
        Err(error) => eprintln!("Error reading input: {}", error),
    }   
    println!("Goodbye!");   
}
