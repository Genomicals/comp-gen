
/*
Node:
    link to parent, option
    suffix link
    string index
*/

use std::{rc::Rc, cell::RefCell, thread::current};
#[derive(Clone)]
#[derive(Default)]
pub struct Node { //string$, suffix_link -> tring$
    parent: Option<Rc<RefCell<Node>>>, //points to immediate parent
    string_index: (usize, usize), //the coordinates of the string this node contains
    children: Vec<Rc<RefCell<Node>>>, //children of this node
    suffix_link: Option<Rc<RefCell<Node>>>, //points to the suffix link
}
impl Node {
    pub fn new() -> Self {
        Node {
            parent: None,
            string_index: (0, 0),
            children: Vec::with_capacity(27), //alphabet + $
            suffix_link: None,
        }
    }


    pub fn get_string(&self, string: &str) -> String {
        String::from(&string[self.string_index.0..self.string_index.1])
    }

    pub fn find_path(&mut self, rc: Rc<RefCell<Node>>, string: &str, index: usize) {
        let current_str = self.get_string(string); //what the current node contains
        println!("current edge: {:?}",current_str);
        let target_str = String::from(&string[index..]); //the string we want to insert
        println!("edge to insert: {:?}",target_str);
        if target_str.contains(&current_str) { //if the target fully contains the current string
            let mut cur_len: usize = 0;
            let rest_str = String::from(&target_str[current_str.len()..]); //rest of the string after current_str
            for child in &self.children {
                if child.borrow().get_string(&string).as_bytes()[0] == rest_str.as_bytes()[0] { //found the next child
                    cur_len = current_str.len(); //reduce memory usage
                    drop(current_str);
                    drop(target_str);
                    drop(rest_str);
                    println!("Moving to child with current edge: {:?}", child.borrow().get_string(string));
                    child.borrow_mut().find_path(child.clone(), string, index + cur_len); //recursion
                    return;
                } 
            }
            println!("no children found that matched first letter of '{:?}'", rest_str);
            //we didn't find a good child, so add a new one
            let mut new_node = Node::new();
            new_node.parent = Some(rc); //set the parent
            new_node.string_index = (index + cur_len, string.len()); //set the start index after the current node's index to the end
            println!("creating new node with edge string of: '{:?}'", new_node.get_string(string));
            self.children.push(Rc::new(RefCell::new(new_node)));
        //branch shares starting character
        } else {
            println!("split the current node into '{:?}' and '{:?}", target_str, current_str);
            let mut i = 0;
            while current_str.as_bytes()[i] != b'$' && target_str.as_bytes()[i] != b'$' && current_str.as_bytes()[i] == target_str.as_bytes()[i]{
                println!("{:?}", current_str.as_bytes()[i]);
                i += 1;
            }

            //initialize new internal node
            let mut new_internal_node = Node::new();
            new_internal_node.parent = self.parent.clone();
            //drop(self.parent);
            let mut new_internal_str = &current_str[0..i];
            new_internal_node.string_index = (index , index + new_internal_str.len());
            println!("new internal node edge: {:?}", new_internal_node.get_string(string));

            //initialize leaf
            let mut new_leaf_node = Node::new();
            new_leaf_node.parent = Some(Rc::new(RefCell::new(new_leaf_node.clone())));
            new_leaf_node.string_index = (index + i, string.len());
            println!("new leaf node edge: {:?}", new_leaf_node.get_string(string));

            //update current node
            let s = self.string_index.0;
            let e = self.string_index.1;
            drop(self.string_index);
            self.parent = Some(Rc::new(RefCell::new(new_internal_node.clone())));
            self.string_index = (s + new_internal_str.len(), e);
            println!("new current node edge: {:?}", self.get_string(string));

            //update the children
            new_internal_node.children.push(Rc::new(RefCell::new(new_leaf_node)));
            new_internal_node.children.push(rc);

            //Endless pit of errors lol wtffff

            // let node: Node = match self.parent {
            //     Some(rc) => {
            //         let node_ref = rc.borrow_mut();
            //         let node_clone = node_ref.clone();
            //         drop(node_ref); // Release the mutable borrow before returning the Node
            //         node_clone
            //     },
            //     None => {
            //         Node::default()
            //     }
            // };

        }

            // let new_internal_node_refCell = Rc::new(RefCell::new(new_internal_node));
            // let mut new_internal_node_option = Some(new_internal_node_refCell.borrow_mut());
            // if let Some(rc_node) = new_internal_node_option {
            //     // Get a reference to the RefCell value
            //     let ref_cell_node = rc_node.parent.as_ref();
            //     ref_cell_node = self.parent.as_ref();
            // }
            // new_internal_node.parent = Some(self.parent);
        // we reach here if target doesn't fully contain current string
        // split the current node


        //current: na
        //target: nana$
        //rest: na$

        //ROOT
        //banana$
        //ana 
        //  na$
        //  $
        //nana$

        //na$

        // if the target string completely contains the current string
        //      check if a child contains the next letter, if so recurse
        //      otherwise split and create a new child
        // else split and create a child

    } 
}

