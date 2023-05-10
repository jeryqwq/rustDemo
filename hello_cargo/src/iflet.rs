
fn main () {
  // 解构 + 覆盖 + 作用域
  let age = Some(30);
  println!("age: {:?}", age);
  if let Some(age) = age {
    println!("age: {}", age);
  }
  println!("age: {:?}", age);

    // 用来解决 Rust 中变量是否有值的问题
    fn plus (num: Option<i32>) -> Option<i32>{
      match num {
          None => None,
          Some(i) => Some(i + 1)
      }
    }
    println!("val: {:?}!", plus(Some(20)));
    println!("val: {:?}!", plus(None));
}
