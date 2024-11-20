fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3i32)
}

fn main() {
    let (x,y,z) = one_two_three();
    
    println!("x = {}, y = {}, z = {}", x, y, z);
}
