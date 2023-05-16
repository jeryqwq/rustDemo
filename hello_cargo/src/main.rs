fn main() {
  let mut s: String = String::from("hello");

  let r1 = &s;
  let r2 = &s;
  println!("{r1} and {r2}");
  // 新编译器中，r1,r2作用域在这里结束

  let r3 = &mut s;
  *r3 = String::from("hello2");
  println!("{}", r3); 
}

