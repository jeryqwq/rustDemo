fn main() {
  fn test<'a>() {
      let r:&'a i32;
      {                     //          |
          let x:&'a i32 = &5;        // -+-- 'b  |
          r = &x;
      }                     // -+       |
      println!("r: {}", r); //          |
  }
  test()
}   
