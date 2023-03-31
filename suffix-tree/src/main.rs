//cargo build
//cargo run --release
//cargo check

//> 
//>
//Co-authored-by: Nathan Balcarcel <nbalcarc@users.noreply.github.com>'

mod node;
use std::{rc::Rc, cell::RefCell};
use node::{Node, TreeConfig};

// fn make_tree() {
    
// }



fn main() {
    let word = "banana$";
    let mut config = TreeConfig::new();
    let root = Node::new(&mut config);
    let rc_refcell_root = Rc::new(RefCell::new(root));

    for i in 0..word.len() {

        println!("New current suffix = {:?}_______________________________________", &word[i..]);
        Node::find_path(rc_refcell_root.clone(), word, i, &mut config);
    }

    Node::print_tree(rc_refcell_root, word);

    return;

    println!("\n\nDoing hops on all suffixes*****************************************************\n\n");
    let word2 = "banana$";
    for i in 0..word2.len() {
        println!("next Suffix = {:?}_____________________________________", &word2[i..]);
       
        let res = Node::node_hops(rc_refcell_root.clone(), word, &word2[i..]);
        match res {
            None => println!("Could not find a node"),
            Some(refer) => println!("Resulting node: {:?}, {:?}, depth {:?}",
                refer.borrow().id,
                refer.borrow().get_string(word),
                refer.borrow().depth,
            ),
        }

    }
    
}
