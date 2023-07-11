use std::fmt::Debug;

pub struct Node<T> {
  pub val: T,
  pub next: Option<Box<Node<T>>>
}
impl<T: Debug> Node<T> {
  pub fn new(val: T) -> Self {
    Node {
      val,
      next: None
    }
  }
  pub fn append(&mut self, value: T) {
    let new_node = Box::new(Node::new(value));
    self.next = Some(new_node);
  }
  pub fn appendNode(&mut self, node: Node<T>) {
    let new_node = Box::new(node);
    self.next = Some(new_node);
  }
  pub fn print(&self) {
    let mut current_node = self;
    loop {
      print!("{:#?}", current_node.val);
      if let Some(next_node) = &current_node.next {
        current_node = next_node;
      } else {
        break;
      }
    }
  }
}
