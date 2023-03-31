//cargo build
//cargo run --release
//cargo check

//> 
//>
//Co-authored-by: Nathan Balcarcel <nbalcarc@users.noreply.github.com>'

mod node;
use std::{rc::Rc, cell::RefCell};
use node::Node;


fn create_tree(string: &str) -> node::Node {
    Node::new()
}


fn main() {
    let word = "mississippi$";
    let root = create_tree(&word);
    let rc_refcell_root = Rc::new(RefCell::new(root));

    for i in 0..word.len() {

        println!("New current suffix = {:?}_______________________________________", &word[i..word.len()]);
        Node::find_path(rc_refcell_root.clone(), word, i);
    }
}
