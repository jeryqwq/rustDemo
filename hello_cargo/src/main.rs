
fn main () {
  let s:&String = &String::from("hello");
  test(&s);
  println!("S2:{}", s);
}

fn test (s: &String) {
 println!("S:{}", s);
}
