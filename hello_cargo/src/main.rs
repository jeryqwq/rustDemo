use std::vec;



fn main() {
  let v1 = vec![1,2,3];
  let v2 = &v1;
  println!("v1: {:?} - {:?}", v1, v2);
}

