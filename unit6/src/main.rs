enum Position {
    Fixed,
    Absolute,
    Relative,
}

fn main() {
    
    let pos:Position = Position::Fixed;
    match pos {
        Position::Fixed => println!("======Fixed"),
        Position::Absolute => println!("======Absolute"),
        Position::Relative => println!("=======Relative"),
    }
    
}
