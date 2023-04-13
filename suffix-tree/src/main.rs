//cargo build
//cargo run --release
//cargo check

//> 
//>
//Co-authored-by: Nathan Balcarcel <nbalcarc@users.noreply.github.com>

mod node;
mod api;

use std::{rc::Rc, cell::RefCell, fs, collections::HashSet};
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


    // collect the input parameters
    let args: clap::ArgMatches = command!()
        .arg(arg!(
            [SEQUENCE] "File containing the sequence to build the tree from."
        ))
        .arg(arg!(
            [ALPHABET] "File containing the alphabet of the sequence."
        ))
        .get_matches(); // run clap

    let sequence_file: String;
    let alphabet_file: String;
    match args.get_one::<String>("SEQUENCE") { //grab the input sequence
        None => {
            panic!("Missing required command-line option: SEQUENCE");
        },
        Some(file) => {
            sequence_file = file.clone();
        }
    }
    match args.get_one::<String>("ALPHABET") { //grab the alphabet
        None => {
            panic!("Missing required command-line option: ALPHABET");
        },
        Some(file) => {
            alphabet_file = file.clone();
        }
    }
    let sequence_filename_index = sequence_file.rfind('/').unwrap_or(0);
    let sequence_filename = String::from(&sequence_file[sequence_filename_index..]);

    // read the input files
    let sequence_raw = fs::read_to_string(sequence_file).expect("Error reading sequence file");
    let alphabet_raw = fs::read_to_string(alphabet_file).expect("Error reading alphabet file");
    let sequence_lines = sequence_raw.lines();

    // parse the input files
    let mut sequence_name: String = String::new();
    let mut sequence = String::new();
    let mut alphabet = HashSet::<char>::new();
    let mut skipped = false; //whether we've skipped the name of the sequence yet
    for line in sequence_lines {
        if line.starts_with(">") { // skip this line but push a new string
            if skipped {
                panic!("Bad sequence syntax");
            } else {
                sequence_name = String::from(&line[1..]);
                skipped = true;
            }
        } else {
            sequence.push_str(line);
        }
    }
    for char in alphabet_raw.chars() {
        if char == ' ' { //skip spaces
            continue;
        } else if char == '\n' || char == '\t' { //break on tabs or newlines
            break;
        } else {
            alphabet.insert(char); //push characters
        }
    }


    // start generating the suffix tree
    let mut interface = Interface::new();
    let tree = interface.make_tree_with_links(&sequence, &alphabet);



    let total_nodes = interface.get_node_count();

    println!("\n\n___________________TREE STATISTICS___________________");
    println!("Total nodes in the tree: {:?}", total_nodes);
    println!("Total leaves in the tree: {:?}", sequence.len() + 1);
    println!("Total internal nodes in the tree: {:?}", total_nodes - (sequence.len() + 1));
    //println!("Average string depth of an internal node: {:?}", interface.average_string_depth());
    //println!("String depth of deepest internal node: {:?}", interface.get_deepest_node_string_depth());
    //println!("Depth of deepest internal node: {:?}", interface.get_deepest_node_depth());
    //println!("Longest exact matching repeat: {:?}", interface.get_longest_repeat());
    interface.DFS_metrics(sequence_filename);


    //let bwt = interface.BWT_index();
    //println!("BWT = {:?}", bwt);

    //Get some node u in the tree
    //let s = &sequence[5..];
    //println!("Finding node: {:?}", s);
    //let u = interface.node_hops(s);
    //interface.display_children(u.clone().unwrap());


    ////Get some node u in the tree
    //let s = &sequence[10..];
    //println!("Finding node: {:?}", s);
    //let u = interface.node_hops(s);
    //interface.display_children(u.clone().unwrap());

    ////print the children of node u
    //println!("\n\nDFS TRAVERSAL++++++++++++++++++++++++++++");
    //interface.DFS(tree.clone());


    //// println!("\n\nDEBUG PRINT++++++++++++++++++++++++++++");
    //// interface.print_tree();


    //if let None = u {
    //    println!("Ran into an error, couldn't find the node!");
    //    return;
    //}

    ////print the children of node u-
    //println!("\n\nCHILDREN++++++++++++++++++++++++++++");
    //interface.display_children(u.unwrap().clone());


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
