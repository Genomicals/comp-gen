
/*
Node:
    link to parent, option
    suffix link
    string index
*/

use std::{rc::Rc, cell::RefCell, thread::current};
#[derive(Clone, Default, PartialEq, Debug)]
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
        let current_str = String::from(self.get_string(string)); //what the current node contains
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
            new_node.parent = Some(rc.clone()); //set the parent
            new_node.string_index = (index + cur_len, string.len()); //set the start index after the current node's index to the end
            println!("creating new node with edge string of: '{:?}'", new_node.get_string(string));
            self.children.push(Rc::new(RefCell::new(new_node)));

        } else { //branch shares starting character
            println!("split the current node into '{:?}' and '{:?}", target_str, current_str);
            let mut split_index = 0;
            while current_str.as_bytes()[split_index] != b'$' && target_str.as_bytes()[split_index] != b'$' && current_str.as_bytes()[split_index] == target_str.as_bytes()[split_index] {
                println!("Splits at char: {:?}", current_str.as_bytes()[split_index]);
                split_index += 1;
            }

            //let parent_rc = rc.borrow_mut().parent.clone().unwrap(); //reference to parent
            let parent_rc = rc.borrow_mut().parent.clone().unwrap(); //reference to parent
            let index_of_cur_in_parent = parent_rc.borrow().children.clone().iter().position(|x| x.as_ptr() == rc.as_ptr()).unwrap();

            //initialize new internal node_____________________________________________________
            let new_internal_node = Node::new(); //the split
            let new_internal_rc = Rc::new(RefCell::new(new_internal_node)); //ref to internal_node
            let new_internal_str = String::from(&current_str[0..split_index]);
            new_internal_rc.borrow_mut().string_index = (index , index + new_internal_str.len());

            new_internal_rc.borrow_mut().parent = Some(parent_rc.clone()); //set the split's parent to the current node's parent
            println!("new internal node edge: {:?}", new_internal_rc.borrow().get_string(string));

            //initialize leaf_____________________________________________________
            let new_leaf_node = Node::new();
            let new_leaf_rc = Rc::new(RefCell::new(new_leaf_node)); //ref to leaf_node
            new_leaf_rc.borrow_mut().parent = Some(new_internal_rc.clone());
            new_leaf_rc.borrow_mut().string_index = (index + split_index, string.len());
            println!("new leaf node edge: {:?}", new_leaf_rc.borrow().get_string(string));

            //update internal node parent to current parent_____________________________________________________
            self.parent = Some(new_internal_rc.clone());

            //update current node edge
            self.string_index = (self.string_index.0 + new_internal_str.len(), self.string_index.1);
            println!("new current node edge: {:?}", self.get_string(string));

            //update the children
            new_internal_rc.borrow_mut().children.push(new_leaf_rc.clone()); //push the new leaf
            new_internal_rc.borrow_mut().children.push(rc.clone()); //push the current node


            //drop(rc);
            //drop(new_internal_rc);
            //drop(new_leaf_rc);

            //update the parent
            //let rc_pointer = rc.as_ptr();
            let index_of_cur_in_parent = parent_rc.borrow().children.clone().iter().position(|x| x.as_ptr() == rc.as_ptr()).unwrap();
            //let index_of_cur_in_parent = parent_rc.try_borrow();
            //println!("{:?}", index_of_cur_in_parent);
            parent_rc.borrow_mut().children.remove(index_of_cur_in_parent);
            parent_rc.borrow_mut().children.push(new_internal_rc);

        }
    } 
}

