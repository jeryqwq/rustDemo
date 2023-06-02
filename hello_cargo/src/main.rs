

fn main() {
  let  z = 20;
  let refZ = & z;
  let sum = |x: i32, y: i32| -> i32 {
    return x + y + *refZ
  };
  println!("{}", sum(3,2));
  let example_closure = |x | x;
  let s = example_closure(String::from("hello"));
  let n = example_closure(5);
}

