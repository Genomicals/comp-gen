use std::{rc::Rc, cell::RefCell, collections::HashSet, fs};
use crate::node::{Node, TreeConfig};

pub struct Interface {
    pub config: TreeConfig,
    pub root: Rc<RefCell<Node>>,
}
impl Interface {
    pub fn new() -> Self {
        let mut config = TreeConfig::new("", HashSet::new());
        let root = Rc::new(RefCell::new(Node::new(&mut config)));
        Interface {
            root: root.clone(),
            config: config,
        }
    }


    /// Creates a suffix tree with the given string and alphabet, includes suffix links
    pub fn make_tree(&mut self, string: &str, alphabet: &HashSet<char>, source_string: usize) -> Rc<RefCell<Node>> {
        //println!("CREATING STRING FROM: {}", string);
        let mut config = TreeConfig::new(&(String::from(string) + "$"), alphabet.clone());
        self.root = Rc::new(RefCell::new(Node::new(&mut config)));
        self.config = config;
        let self_rc = self.root.clone();
        self.root.borrow_mut().parent = Some(self_rc.clone());
        self.root.borrow_mut().suffix_link = Some(self_rc);
        let mut cur = Node::find_path(self.root.clone(), 0, source_string, &mut self.config);

        for i in 1..string.len() {
            cur = Node::suffix_link_insert(cur.clone(), i, source_string, &mut self.config);
        }

        return self.root.clone();
    } 


    /// Adds another string to this suffix tree
    pub fn add_string(&mut self, string: &str, source_string: usize) {
        //println!("CREATING STRING FROM: {}", string);
        self.config.strings.push(String::from(string) + "$"); //add the new string to the list
        let mut cur = Node::find_path(self.root.clone(), 0, source_string, &mut self.config); //insert whole string, retrieve a pointer in the process

        for i in 1..string.len() {
            cur = Node::suffix_link_insert(cur.clone(), i, source_string, &mut self.config);
        }
    }


    /// Color the tree nodes
    pub fn color_tree(&mut self) {
        //println!("coloring");
        Interface::color_tree_recursive(self.root.clone());
    }
    fn color_tree_recursive(node: Rc<RefCell<Node>>) {
        println!("recursion called!");
        let children = node.borrow().children.clone();
        //println!("um, here?");

        if children.len() == 0 { //don't change the color of leaves
            //println!("found a leaf");
            return;
        }

        let mut color = children[0].borrow().node_color; //will inherit a pure or mixed color from children
        if color == -1 {
            node.borrow_mut().node_color = -1; //if the first child is mixed, then we must be mixed
            //println!("early return");
            return;
        }
        for child in children {
            Interface::color_tree_recursive(child.clone());
            if child.borrow().node_color != color {
                color = -1;
                //println!("early return");
                node.borrow_mut().node_color = -1; //if any node color doesn't match up with the initial, then this node must be mixed color
                //return;
            } else {
                //println!("cur color: {}, child: {}", color, child.borrow().node_color);
            }
        }

        
        //println!("Reached node, {}", rc.borrow().as_string(config));

        //for child in rc.borrow().children.clone() {
        //    Node::print_tree(child, config);
        //    ////println!("Returned to node {:?}", rc.borrow().id);
        //}

        node.borrow_mut().node_color = color;
    }


    /// Returns a vector of fingerprints for each string in the tree
    pub fn get_fingerprints(&mut self) -> Vec<Vec<String>> {
        let mut fingerprints = Vec::with_capacity(self.config.strings.len());
        //let mut fingerprint_nodes = Vec::with_capacity(self.config.strings.len()); //helps keep track of fingerprints while discovering them
        let mut fingerprint_nodes = vec![(0, Vec::new()); self.config.strings.len()];

        Interface::get_fingerprints_recursive(self.root.clone(), &mut fingerprint_nodes);
        let collected_nodes: Vec<Vec<Rc<RefCell<Node>>>> = fingerprint_nodes.into_iter().map(|elem| elem.1).collect(); //into_iter() consumes the vector, no borrow errors like you get with iter()

        // although this is a triple-nested loop, ultimately the runtime complexity shouldn't be affected because these for-loops are limited in how large they can get
        for i in 0..collected_nodes.len() { //iterate through all string indices
            let mut new_fingerprints = Vec::with_capacity(collected_nodes[i].len());

            for j in &collected_nodes[i] { //iterate through all nodes collected for the current string index
                //let cur_string = j.borrow().st
                let cur_string = Node::reconstruct_string(j.clone(), &self.config);
                //let children = j.borrow().children;

                for child in &j.borrow().children { //iterate through all of this node's children
                    if child.borrow().node_color != i as isize { //ignore any children that don't match the color we're looking for
                        continue;
                    }
                    //println!("there are in fact {} colors", self.config.strings.len());
                    //println!("1: {}", i);
                    //println!("3: {}", child.borrow().source_string);
                    //println!("2: {}", child.borrow().string_index.0);
                    let first_char = self.config.strings[i].as_bytes()[child.borrow().string_index.0] as char;
                    let mut new_str = cur_string.clone();
                    new_str.push(first_char);
                    new_fingerprints.push(new_str); //push this child's fingerprint to the list of new fingerprints
                }
            }
            fingerprints.push(new_fingerprints); //push all the fingerprints collected for string i onto the return vector
        }
        fingerprints
    }
    fn get_fingerprints_recursive(node: Rc<RefCell<Node>>, fingerprints: &mut Vec<(usize, Vec<Rc<RefCell<Node>>>)>) {
        let children = node.borrow().children.clone();

        if children.len() == 0 { //ignore any leaf nodes
            return;
        }

        for child in children {
            if child.borrow().node_color != -1 { //found an unmixed child
                let color = child.borrow().node_color as usize;
                //println!("color: {}", color);
                if fingerprints[color].0 > node.borrow().string_depth { //already obtained deeper fingerprints for this color
                    continue;
                }
                if fingerprints[color].0 < node.borrow().string_depth { //found deeper fingerprints than what we have, remove all collected nodes in favor of new ones
                    fingerprints[color] = (node.borrow().string_depth, Vec::new()); //this will automatically activate the following if-condition as well
                }
                if fingerprints[color].0 == node.borrow().string_depth { //add the current fingerprint to the existing list of equally-depthed node candidates
                    fingerprints[color].1.push(child.clone());
                }
            }
        }

    }


    pub fn get_node_count(&self) -> usize {
        self.config.next_id
    }


    /// Displays all of node's children from left to right, for debugging
    pub fn display_children(&mut self, node: Rc<RefCell<Node>>) {
        let children = node.borrow().children.clone();

        if children.len() == 0 {
            println!("No children found!");
            return;
        }
        
        for child in &children {
            let child_indices = child.borrow().string_index;
            println!("ID: {:?}, Depth: {:?}, Edge: {:?}",
                child.borrow().id,
                child.borrow().depth,
                &self.config.strings[child.borrow().source_string][child_indices.0..child_indices.1],
            );
        }
    }

    
    ///// Produce the metrics of the suffix tree via DFS
    //pub fn DFS_metrics(&self, file_name: String) -> String {
    //    let mut leaf_vec = Vec::new();
    //    let (total_depth, leaves, stringest) = Interface::DFS_metrics_recursive(self.root.clone(), 0, &mut leaf_vec, self.root.clone());

    //    let total_nodes = self.get_node_count();
    //    let internal_nodes = total_nodes - (self.config.string.len() + 1);
    //    let average_depth: f64 = total_depth as f64 / internal_nodes as f64;
    //    
    //    // first add indices to all the id's
    //    let mut indexed_leaves: Vec<(usize, usize)> = Vec::with_capacity(self.config.string.len()); //id, index
    //    for i in 0..leaves.len() {
    //        indexed_leaves.push((leaves[i], i));
    //    }
    //    indexed_leaves.sort_unstable();

    //    // then normalize all the id's
    //    let mut revised_indexed_leaves = Vec::with_capacity(self.config.string.len()); //will normalize the id's so they're continuous without gaps
    //    for i in 0..indexed_leaves.len() {
    //        revised_indexed_leaves.push((i, indexed_leaves[i].1));
    //    }
    //    revised_indexed_leaves.sort_unstable_by(|left, right| left.1.partial_cmp(&right.1).unwrap()); //sort by index

    //    // push the indices in the correct order
    //    let mut bwt_string = String::with_capacity(self.config.string.len());
    //    let mut bwt_string_file = String::with_capacity(self.config.string.len() * 2);
    //    let the_str = self.config.string.as_bytes();
    //    for i in 0..revised_indexed_leaves.len() {
    //        if revised_indexed_leaves[i].0 == 0 {
    //            bwt_string.push(the_str[self.config.string.len() - 1] as char);
    //            bwt_string_file.push(the_str[self.config.string.len() - 1] as char);
    //            bwt_string_file.push('\n');
    //        } else {
    //            bwt_string.push(the_str[revised_indexed_leaves[i].0 - 1] as char);
    //            bwt_string_file.push(the_str[revised_indexed_leaves[i].0 - 1] as char);
    //            bwt_string_file.push('\n');
    //        }
    //    }

    //    // print
    //    println!("Average string depth of an internal node: {:?}", average_depth);
    //    println!("String depth of deepest internal node: {:?}", stringest.borrow().string_depth);
    //    let full_file_name = String::from("output/") + &file_name + "_BWT.txt";
    //    fs::write(&full_file_name, &bwt_string_file).expect("Unable to write file");
    //    bwt_string
    //}


    /// Depth-first traversal metrics, recursive
    pub fn DFS_metrics_recursive(rc: Rc<RefCell<Node>>, total_depth: usize, leaves: &mut Vec<usize>, mut stringest: Rc<RefCell<Node>>) -> (usize, &mut Vec<usize>, Rc<RefCell<Node>>) {
        let children = rc.borrow().children.clone();

        // update the string depth if no children, i.e. an internal node
        if children.len() > 0 && rc.borrow().string_depth > stringest.borrow().string_depth {
            stringest = rc.clone();
        }

        // start recursive process
        let mut new_depth = total_depth + rc.borrow().string_depth;
        let mut new_stringest;
        if children.len() == 0 { //if this node is a leaf
            leaves.push(rc.borrow().id) //collect leaf ids
        }
        for child in children {
            (new_depth, _, new_stringest) = Interface::DFS_metrics_recursive(child, new_depth, leaves, stringest.clone()); //collect metrics for each child

            if new_stringest.borrow().string_depth > stringest.borrow().string_depth {
                stringest = new_stringest.clone();
            }
        }

        (new_depth, leaves, stringest)
    }

  
    /// Prints the tree, for debugging
    pub fn print_tree(&self) {
        println!("here");
        Node::print_tree(self.root.clone(), &self.config);
    }
}

