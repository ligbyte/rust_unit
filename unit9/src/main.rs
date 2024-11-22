fn main() {
    // 向量
    let mut v: Vec<&str> = Vec::new();
    v.push("hello");
    v.push("world");
    v.push("!");
    println!("{:?}", v);
    v.remove(2);
    println!("{:?}", v);
}
