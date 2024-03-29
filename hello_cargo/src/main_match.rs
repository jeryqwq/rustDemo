#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
        println!("Lucky penny!");
        1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
      None => None,
      Some(i) => Some(i + 1),
  }
}

fn main() {
  println!("val: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("val: {:?}", six);
  println!("val: {:?}", none);
}




enum Action {
  Say(String),
  MoveTo(i32, i32),
  ChangeColorRGB(u16, u16, u16),
}
fn main() {
  let sayHello = Action::Say("hello Chencc".to_string());
  match sayHello {
      Action::Say(text) => {
        println!("context: {}!", text);
      },
      _ => ()
  }
}

