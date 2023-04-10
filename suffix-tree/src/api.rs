use std::{rc::Rc, cell::RefCell, collections::HashSet};
use crate::node::{Node, TreeConfig};

pub struct Interface {
    pub config: TreeConfig,
    pub root: Rc<RefCell<Node>>,
    pub deepest_node: Rc<RefCell<Node>>,
}
impl Interface {
    pub fn new() -> Self {
        let mut config = TreeConfig::new("", HashSet::new());
        let root = Rc::new(RefCell::new(Node::new(&mut config)));
        Interface {
            root: root.clone(),
            config: config,
            deepest_node: root.clone(),

        }
    }

    /// Creates a ST with the given string
    pub fn make_tree(&mut self, string: &str, alphabet: &HashSet<char>) -> Rc<RefCell<Node>> {
        let mut config = TreeConfig::new(&(String::from(string) + "$"), alphabet.clone());
        self.root = Rc::new(RefCell::new(Node::new(&mut config)));
        self.config = config;

        for i in 0..self.config.string.len() {
            println!("Next suffix to insert===================: {:?}", &self.config.string[i..]);
            let node = Node::find_path(self.root.clone(), i, &mut self.config);
            if node.borrow().depth > self.deepest_node.borrow().depth {
                self.deepest_node = node.clone();
            }
        }

        return self.root.clone();
    }

    pub fn make_tree_with_links(&mut self, string: &str, alphabet: &HashSet<char>) -> Rc<RefCell<Node>> {
        let mut config = TreeConfig::new(&(String::from(string) + "$"), alphabet.clone());
        self.root = Rc::new(RefCell::new(Node::new(&mut config)));
        self.config = config;
        println!(">>>> Creating tree for {}", &self.config.string);
        let self_rc = self.root.clone();
        self.root.borrow_mut().parent = Some(self_rc.clone());
        self.root.borrow_mut().suffix_link = Some(self_rc);
        println!("Inserted root");
        let mut cur = Node::find_path(self.root.clone(), 0, &mut self.config);
        println!("Inserted first suffix: {:?}", &self.config.string[0..]);

        for i in 1..self.config.string.len() {
            println!("Next suffix to insert=== {}: {:?}", i, &self.config.string[i..]);
            cur = Node::suffix_link_insert(cur.clone(), i, &mut self.config);
            println!("node added");
        }

        return self.root.clone();
    } 


    pub fn get_node_count(&self) -> usize {
        self.config.next_id
    }


    pub fn get_deepest_node_depth(&self) -> usize {
        let deepest = self.deepest_node.clone();
        let parent = deepest.borrow().parent.clone().unwrap();
        let depth = parent.borrow().string_depth;
        depth
    }

    pub fn get_longest_repeat(&self) -> String {
        let deepest = self.deepest_node.clone();
        let ret: String = deepest.borrow().parent.clone().unwrap().borrow().get_string(&self.config);
        ret
    }

    pub fn average_string_depth(&self) -> usize {
        let mut totals: Vec<usize> = Vec::new();
        Interface::average_string_depth_recursive(self.root.clone(), &mut totals);
        let sum: usize = totals.iter().sum();
        return sum / totals.len();
    }


    pub fn average_string_depth_recursive(rc: Rc<RefCell<Node>>, totals: &mut Vec<usize>) {
        totals.push(rc.borrow().string_depth);
        let children = rc.borrow().children.clone();
        for child in children {
            Interface::average_string_depth_recursive(child, totals);
        }
    }


    /// Displays all of node's children from left to right
    pub fn display_children(&mut self, node: Rc<RefCell<Node>>) {
        let children = node.borrow().children.clone();

        if children.len() == 0 {
            println!("No children found!");
            return;
        }
        
        for child in &children {
            println!("ID: {:?}, Depth: {:?}, Edge: {:?}",
                child.borrow().id,
                child.borrow().depth,
                child.borrow().get_string(&self.config)
            );
        }
    }

    /// Depth-first traversal printing
    pub fn DFS(&self, rc: Rc<RefCell<Node>>) {

        //print node
        println!("ID: {:?}, Depth: {:?}, Edge: {:?}",
            rc.borrow().id,
            rc.borrow().depth,
            rc.borrow().get_string(&self.config)
        );

        //sort the children alphabetically
        let children = rc.borrow().children.clone();
        //children.sort_by(|x, y| { //alphabetically sort the list of children
        //    if x.borrow().get_string(&self.string) > y.borrow().get_string(&self.string) {
        //        std::cmp::Ordering::Greater
        //    } else {
        //        std::cmp::Ordering::Less
        //    }
        //});
        
        for child in children {
            self.DFS(child);
        }
    }

    pub fn BWT_index(&self) -> String {

        let mut leaves: Vec<usize> = Vec::with_capacity(self.config.string.len());
        let s = self.config.string.as_bytes();
        let mut bwt = String::from("");
        let mut rc: Rc<RefCell<Node>> = self.root.clone();

        //println!("string = {:?}", s);
        
        //traverse the leaves and add them to b vector
        self.add_to_b(rc, &mut leaves);
        //println!("leaves = {:?}", leaves);

        let mut sorted = leaves.clone();
        sorted.sort();
        //println!("sorted = {:?}", sorted);

        let mut b: Vec<usize> = Vec::with_capacity(self.config.string.len());

        for i in leaves {
            let index = sorted.iter().position(|&r| r == i).unwrap();
            b.push(index);
           // println!("pushed {:?} into b", index);
        }

        for i in b {
            //println!("index is {:?}", i);
            
            if (i == 0) {
                bwt += "$";
                //println!("pushed $ into bwt");
            } else {
                
                bwt += std::str::from_utf8(&[s[i-1]]).unwrap();
                //println!("pushed {:?} into bwt", std::str::from_utf8(&[s[i-1]]).unwrap());
            }
        }
        return bwt;
    }

    fn add_to_b(&self, rc: Rc<RefCell<Node>>, leaves: &mut Vec<usize>) {

        let children = rc.borrow().children.clone();

        if children.len() == 0 {
            leaves.push(rc.borrow().id);
            return;
        }
        
        for child in children {
            self.add_to_b(child, leaves);
        }
        return;
    }

  
    /// Prints the tree, for debugging
    pub fn print_tree(&self) {
        Node::print_tree(self.root.clone(), &self.config);
    }


    /// Internal function for hopping
    pub fn node_hops(&mut self, string: &str) -> Option<Rc<RefCell<Node>>> {
        Node::node_hops(self.root.clone(), string, &mut self.config)
    }
}

