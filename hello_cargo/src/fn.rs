
fn main () { // 引用传递 , 如果是借用无法在内部堆栈后的外部使用 
  let s:&String = &String::from("hello");
  test(s); // & 可加可不加 
  println!("S2:{}", s);
}

fn test (s: &String) {
 println!("S:{}", s);
}
