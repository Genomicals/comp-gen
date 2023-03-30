
/*
Node:
    link to parent, option
    suffix link
    string index
*/

use std::{rc::Rc, cell::RefCell};

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
        let target_str = String::from(&string[index..]); //the string we want to insert
        if target_str.contains(&current_str) { //if the target fully contains the current string
            let mut cur_len: usize = 0;
            let rest_str = String::from(&target_str[current_str.len()..]); //rest of the string after current_str
            for child in &self.children {
                if child.borrow().get_string(&string).as_bytes()[0] == rest_str.as_bytes()[0] { //found the next child
                    cur_len = current_str.len(); //reduce memory usage
                    drop(current_str);
                    drop(target_str);
                    drop(rest_str);
                    child.borrow_mut().find_path(child.clone(), string, index + cur_len); //recursion
                    return;
                } 
            }
            //we didn't find a good child, so add a new one
            let mut new_node = Node::new();
            new_node.parent = Some(rc); //set the parent
            new_node.string_index = (index + cur_len, string.len()); //set the start index after the current node's index to the end
            self.children.push(Rc::new(RefCell::new(new_node)));
        }
        // we reach here if target doesn't fully contain current string
        // split the current node
        //println!("{:?}", "sup");


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

