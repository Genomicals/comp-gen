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
        let mut cur = Node::find_path(self.root.clone(), 0, source_string, &mut self.config);

        for i in 1..string.len() {
            cur = Node::suffix_link_insert(cur.clone(), i, source_string, &mut self.config);
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
        Node::print_tree(self.root.clone(), &self.config);
    }
}

