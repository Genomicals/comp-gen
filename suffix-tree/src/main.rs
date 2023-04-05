//cargo build
//cargo run --release
//cargo check

//> 
//>
//Co-authored-by: Nathan Balcarcel <nbalcarc@users.noreply.github.com>

mod node;
mod api;

use std::{rc::Rc, cell::RefCell, fs};
use node::{Node, TreeConfig};
use api::Interface;
use clap::{arg, command};


/// Used to record the name of a string
#[derive(Debug)]
struct NamedString {
    name: String,
    str: String,
}
impl NamedString {
    fn name(name: &str) -> Self {
        NamedString { name: String::from(name), str: String::new() }
    }
}


fn main() {


    // // Process the arguments with clap
    // let args: clap::ArgMatches = command!()
    //     .arg(arg!(
    //         [FILE] "File containing the string to build the tree for"
    //     ))
    //     .arg(arg!(
    //         [ALPHABET] "File containing the alphabet of the string"
    //     ))
    //     .get_matches(); // run clap

    // let string_file: String;
    // let alphabet_file: String;
    // match args.get_one::<String>("FILE") { //grab the input str
    //     None => {
    //         panic!("Missing required command-line option: FILE");
    //     },
    //     Some(file) => {
    //         string_file = file.clone();
    //     }
    // }
    // match args.get_one::<String>("ALPHABET") { //grab the alphabet
    //     None => {
    //         panic!("Missing required command-line option: ALPHABET");
    //     },
    //     Some(file) => {
    //         alphabet_file = file.clone();
    //     }
    // }

    // let string_string = fs::read_to_string(string_file).expect("Error reading file");
    // let alphabet_string = fs::read_to_string(alphabet_file).expect("Error reading file");

    // let string_lines = string_string.lines();
    // let mut string_vec: Vec<NamedString> = Vec::new();
    // for line in string_lines {
    //     if line.starts_with(">") { // skip this line but push a new string
    //         string_vec.push(NamedString::name(&line[1..line.len()])); //copy from the 1st index to the end
    //         continue;
    //     } else {
    //         string_vec.last_mut().expect("Input strings were in the wrong format").str.push_str(line);
    //     }
    // }

    //let word = "AATTTTACTTTTAA";
    let word = "AAAAAAA";

    let mut interface = Interface::new();
      
    let tree = interface.make_tree_with_links(word, "nana");

    // println!("\n\nPRINTTREE++++++++++++++++++++++++++++");
    // interface.print_tree();

    let total_nodes = interface.get_node_count();

    println!("\n\n___________________TREE STATISTICS___________________");
    println!("Total nodes in the tree: {:?}", total_nodes);
    println!("Total leaves in the tree: {:?}", word.len() + 1);
    println!("Total internal nodes in the tree: {:?}", total_nodes - (word.len() + 1));
    println!("Average string depth of an internal node: {:?}", interface.average_string_depth());
    println!("String depth of deepest internal node: {:?}", interface.get_deepest_node_depth());
    println!("Longest exact matching repeat: {:?}", interface.get_longest_repeat());


    let bwt = interface.BWT_index();
    println!("BWT = {:?}", bwt);

    //Get some node u in the tree
    let s = &word[5..];
    println!("Finding node: {:?}", s);
    let u = interface.node_hops(s);
    interface.display_children(u.clone().unwrap());


    //Get some node u in the tree
    let s = &word[10..];
    println!("Finding node: {:?}", s);
    let u = interface.node_hops(s);
    interface.display_children(u.clone().unwrap());

    //print the children of node u
    println!("\n\nDFS TRAVERSAL++++++++++++++++++++++++++++");
    interface.DFS(tree.clone());


    // println!("\n\nDEBUG PRINT++++++++++++++++++++++++++++");
    // interface.print_tree();


    if let None = u {
        println!("Ran into an error, couldn't find the node!");
        return;
    }

    //print the children of node u-
    println!("\n\nCHILDREN++++++++++++++++++++++++++++");
    interface.display_children(u.unwrap().clone());


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
