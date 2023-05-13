use std::f32::consts::E;


fn main () {
  // 1. 当生命周期中只有一个参数时，编译器会为参数和返回值默认添加'a
  // 2. 当参数是&self时，同理
  // 3. 当参数为多个时，需要手动制定生命周期
//   fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     return &s
// }
//   println!("{:?}",first_word("abc"));



  let str = String::from("abc");
  let str2 = "abcd";
  let longer = getLonger(str.as_str(), str2);
  println!("long:{}", longer)
}

fn getLonger<'b> (a:&'b str, b:&'b str) -> &'b str {
  if(a.len() > b.len()) {
    a
  }else {
      b
  }
}



/**
#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

fn main() {
    let mut foo  = Foo;
    let loan = foo.mutate_and_share();
    foo.share(); //cannot borrow `foo` as immutable because it is also borrowed as mutable
    println!("{:?}", loan);
}

 */
