//cargo build
//cargo run --release
//cargo check

//> 
//>
//Co-authored-by: Nathan Balcarcel <nbalcarc@users.noreply.github.com>

mod node;
mod api;

use std::{rc::Rc, cell::RefCell};
use node::{Node, TreeConfig};
use api::Interface;


fn main() {

    let word = "banana";
    let mut interface = Interface::new();

    let tree = interface.make_tree(word);

    //Get some node u in the tree
    let u = interface.node_hops("na");
    if let None = u {
        println!("Ran into an error, couldn't find the node!");
        return;
    }

    //print the children of node u
    println!("\n\nCHILDREN++++++++++++++++++++++++++++");
    interface.display_children(u.unwrap().clone());

    //print the children of node u
    println!("\n\nDFS TRAVERSAL++++++++++++++++++++++++++++");
    interface.DFS(tree.clone());



    

    


    // println!("\n\nDoing tree printing*****************************************************\n\n");

    // Node::print_tree(rc_refcell_root.clone(), word);

    // println!("\n\nDoing hops on all suffixes*****************************************************\n\n");
    // let word2 = "banana$";
    // for i in 0..word2.len() {
    //     println!("next Suffix = {:?}_____________________________________", &word2[i..]);
       
    //     let res = Node::node_hops(rc_refcell_root.clone(), word, &word2[i..]);
    //     match res {
    //         None => println!("Could not find a node"),
    //         Some(refer) => println!("Resulting node: {:?}, {:?}, depth {:?}",
    //             refer.borrow().id,
    //             refer.borrow().get_string(word),
    //             refer.borrow().depth,
    //         ),
    //     }

    // }
    
}
