fn main() {

  const MAX_NUM: i32 = 10; 
    let mut num: i32 = 0;

  if num < 5 {
      println!("{}", "num < 5");
  }

    
    while num < MAX_NUM {
        num += 1;    
        println!("{}", num);
    }
    

    if num > 5 {
        println!("{}", "num > 5");
    }
    
}
