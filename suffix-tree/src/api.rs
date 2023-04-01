use std::{rc::Rc, cell::RefCell};
use crate::node::{Node, TreeConfig};

pub struct Interface {
    pub string: String,
    pub root: Rc<RefCell<Node>>,
    pub config: TreeConfig,
    pub deepest_node: Rc<RefCell<Node>>,
}
impl Interface {
    pub fn new() -> Self {
        let mut config = TreeConfig::new("");
        let root = Rc::new(RefCell::new(Node::new(&mut config)));
        Interface {
            string: String::from(""),
            root: root.clone(),
            config: config,
            deepest_node: root.clone(),

        }
    }

    /// Creates a ST with the given string
    pub fn make_tree(&mut self, string: &str, alphabet: &str) -> Rc<RefCell<Node>> {
        self.string = String::from(string) + "$";
        let mut config = TreeConfig::new(&alphabet);
        self.root = Rc::new(RefCell::new(Node::new(&mut config)));
        self.config = config;

        for i in 0..self.string.len() {
            //println!("Next suffix to insert===================: {:?}", &self.string[i..]);
            let node = Node::find_path(self.root.clone(), &self.string, i, &mut self.config);
            if node.borrow().depth > self.deepest_node.borrow().depth {
                self.deepest_node = node.clone();
            }
        }

        return self.root.clone();
    }

    pub fn make_tree_with_links(&mut self, string: &str, alphabet: &str) -> Rc<RefCell<Node>> {
        self.string = String::from(string) + "$";
        let mut config = TreeConfig::new(&alphabet);
        self.root = Rc::new(RefCell::new(Node::new(&mut config)));
        let self_rc = self.root.clone();
        //self.root.borrow_mut().parent = Some(Rc::downgrade(&self_rc));
        self.root.borrow_mut().parent = Some(self_rc.clone());
        self.root.borrow_mut().suffix_link = Some(self_rc);
        println!("creating root");
        let mut cur = Node::find_path(self.root.clone(), string, 0, &mut config);
        println!("inserted first node");

        for i in 1..self.string.len() {
            println!("Next suffix to insert===: {:?}", &self.string[i..]);
            cur = Node::suffix_link_insert(cur.clone(), string, i, &mut config);
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
        let ret: String = deepest.borrow().parent.clone().unwrap().borrow().get_string(&self.string);
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
                child.borrow().get_string(&self.string)
            );
        }
    }

    /// Depth-first traversal printing
    pub fn DFS(&self, rc: Rc<RefCell<Node>>) {

        //print node
        println!("ID: {:?}, Depth: {:?}, Edge: {:?}",
            rc.borrow().id,
            rc.borrow().depth,
            rc.borrow().get_string(&self.string)
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

        let mut leaves: Vec<usize> = Vec::with_capacity(self.string.len());
        let s = self.string.as_bytes();
        let mut bwt = String::from("");
        let mut rc: Rc<RefCell<Node>> = self.root.clone();

        //println!("string = {:?}", s);
        
        //traverse the leaves and add them to b vector
        self.add_to_b(rc, &mut leaves);
        //println!("leaves = {:?}", leaves);

        let mut sorted = leaves.clone();
        sorted.sort();
        //println!("sorted = {:?}", sorted);

        let mut b: Vec<usize> = Vec::with_capacity(self.string.len());

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
        Node::print_tree(self.root.clone(), &self.string);
    }


    /// Internal function for hopping
    pub fn node_hops(&mut self, string: &str) -> Option<Rc<RefCell<Node>>> {
        Node::node_hops(self.root.clone(), &self.string, string)
    }
}

