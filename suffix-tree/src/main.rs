//cargo build
//cargo run --release
//cargo check
mod node;
use node::Node;


fn create_tree(string: &str) -> node::Node {
    Node::new()
}


fn main() {
    println!("Hello, world!");
}
