use std::{rc::Rc, cell::RefCell};
use crate::node::{Node, TreeConfig};

pub struct Interface {
    pub string: String,
    pub root: Rc<RefCell<Node>>,
}
impl Interface {
    pub fn new() -> Self {
        Interface {
            string: String::from(""),
            root: Rc::new(RefCell::new(Node::new(&mut TreeConfig::new()))),
        }
    }


    /// Creates a ST with the given string
    pub fn make_tree(&mut self, string: &str) -> Rc<RefCell<Node>> {
        self.string = String::from(string) + "$";
        let mut config = TreeConfig::new();
        self.root = Rc::new(RefCell::new(Node::new(&mut config)));

        for i in 0..self.string.len() {
            Node::find_path(self.root.clone(), &self.string, i, &mut config);
        }

        return self.root.clone();
    }


    /// Displays all of node's children from left to right
    pub fn display_children(&mut self, node: Rc<RefCell<Node>>) {
        let mut children = node.borrow().children.clone();

        if children.len() == 0 {
            println!("No children found!");
            return;
        }

        children.sort_by(|x, y| { //alphabetically sort the list of children
            if x.borrow().get_string(&self.string) > y.borrow().get_string(&self.string) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        
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
        let mut children = rc.borrow().children.clone();
        children.sort_by(|x, y| { //alphabetically sort the list of children
            if x.borrow().get_string(&self.string) > y.borrow().get_string(&self.string) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        
        for child in children {
            self.DFS(child);
        }
    }

    


    /// Internal function for hopping
    pub fn node_hops(&mut self, string: &str) -> Option<Rc<RefCell<Node>>> {
        Node::node_hops(self.root.clone(), &self.string, string)
    }
}

