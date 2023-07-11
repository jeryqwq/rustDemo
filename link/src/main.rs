mod link;
use link::Node;
fn main() {
    println!("Hello, world!");
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let mut node4 = Node::new(4);
    let mut node5 = Node::new(5);
    node5.append(6);
    node4.appendNode(node5);
    node3.appendNode(node4);
    node2.appendNode(node3);
    node1.appendNode(node2);
    node1.print();
}
