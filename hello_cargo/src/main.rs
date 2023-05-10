pub struct  Rang {
  width: i32,
  height: i32
}
impl Rang {
    fn area (&self) -> i32 {
      return self.width * self.height
    }
    fn new (width: i32, height: i32) -> Self {
      return Self{
        width,
        height
      }
    }
}
fn main () {
  let rang = Rang::new(20, 20);
  println!("area: {:?}", rang.area());
}
