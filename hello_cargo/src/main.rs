

fn main() {
  let mut arr = vec![3;5];
  let ret = test(&arr);
  println!("{:?}", ret);
  println!("{:?}", arr);

}

fn test (arr: &Vec<i32>) -> Vec<i32> {
  let mut ret = arr.clone();
  for i in &mut ret {
      *i *= 2;
  }
  return ret;
}
