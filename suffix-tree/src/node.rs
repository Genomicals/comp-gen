
/*
Node:
    link to parent, option
    suffix link
    string index
*/

use std::{rc::Rc, cell::RefCell, thread::current};



#[derive(Clone, Default, PartialEq, Debug)]
pub struct TreeConfig {
    pub next_id: usize,
}
impl TreeConfig {
    pub fn new() -> Self {
        TreeConfig {next_id: 0}
    }

    pub fn next(&mut self) -> usize {
        self.next_id += 1;
        self.next_id - 1
    }
}


#[derive(Clone, Default, PartialEq, Debug)]
pub struct Node { //string$, suffix_link -> tring$
    pub id: usize,
    pub parent: Option<Rc<RefCell<Node>>>, //points to immediate parent
    pub string_index: (usize, usize), //the coordinates of the string this node contains
    pub children: Vec<Rc<RefCell<Node>>>, //children of this node
    pub suffix_link: Option<Rc<RefCell<Node>>>, //points to the suffix link
    pub depth: u32,
}
impl Node {
    pub fn new(config: &mut TreeConfig) -> Self {
        Node {
            id: config.next(),
            parent: None,
            string_index: (0, 0),
            children: Vec::with_capacity(27), //alphabet + $
            suffix_link: None,
            depth: 0
        }
    }


    /// Depth-first print
    pub fn print_tree(rc: Rc<RefCell<Node>>, string: &str) {
        println!("Reached node, id: {:?}, depth: {:?}, edge: {:?}",
            rc.borrow().id,
            rc.borrow().depth,
            rc.borrow().get_string(string),
        );

        for child in rc.borrow().children.clone() {
            Node::print_tree(child, string);
            //println!("Returned to node {:?}", rc.borrow().id);
        }
    }


    /// Depth-first depth update
    pub fn update_depth_recursive(rc: Rc<RefCell<Node>>) {
        for child in rc.borrow().children.clone() {
            child.borrow_mut().depth += 1;
            Node::update_depth_recursive(child);
        }
    }


    /// Get the string this node contains
    pub fn get_string(&self, string: &str) -> String {
        String::from(&string[self.string_index.0..self.string_index.1])
    }


    /// Inserts the given suffix (of 'string' starting at 'index') under the given node
    pub fn find_path(rc: Rc<RefCell<Node>>, string: &str, index: usize, config: &mut TreeConfig) {
        let current_str = String::from(rc.borrow().get_string(string)); //what the current node contains
        //println!("current edge^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^: {:?}",current_str);
        let target_str = String::from(&string[index..]); //the string we want to insert
        //println!("edge to insert^^^^^^^^^^^^^^^^^^^^^^^^^^: {:?}",target_str);
        if target_str.starts_with(&current_str) { //if the target fully contains the current string
            let mut cur_len: usize = 0;
            let rest_str = String::from(&target_str[current_str.len()..]); //rest of the string after current_str
            let rc_children = rc.borrow().children.clone();
            for child in &rc_children {
                if child.borrow().get_string(&string).as_bytes()[0] == rest_str.as_bytes()[0] { //found the next child
                    cur_len = current_str.len(); //reduce memory usage
                    drop(current_str);
                    drop(target_str);
                    drop(rest_str);
                    // println!("Moving to child with current edge: {:?}", child.borrow().get_string(string));
                    Node::find_path(child.clone(), string, index + cur_len, config); //recursion
                    return;
                } 
            }
            // println!("no children found that matched first letter of {:?}", rest_str);
            //we didn't find a good child, so add a new one
            let mut new_node = Node::new(config);
            new_node.parent = Some(rc.clone()); //set the parent
            new_node.depth = rc.borrow().depth + 1;
            new_node.string_index = (index + cur_len, string.len()); //set the start index after the current node's index to the end
            // println!("creating new node with edge string of: {:?}", new_node.get_string(string));
            let new_node_rc = Rc::new(RefCell::new(new_node));
            rc.borrow_mut().children.push(new_node_rc.clone());
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!RESULTS!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("Cur edge: {:?}", rc.borrow().get_string(string));
            println!("Cur depth: {:?}", rc.borrow().depth);
            println!("Cur count: {:?}", rc.borrow().children.len());
            println!("New edge: {:?}", new_node_rc.borrow().get_string(string));
            println!("New depth: {:?}", new_node_rc.borrow().depth);
            println!("New count: {:?}", new_node_rc.borrow().children.len());
            println!("---ENDRESULTS---");

        } else { //branch shares starting character
            // println!("split the current node into {:?} and {:?}", target_str, current_str);
            let mut split_index = 0;
            while current_str.as_bytes()[split_index] != b'$' && target_str.as_bytes()[split_index] != b'$' && current_str.as_bytes()[split_index] == target_str.as_bytes()[split_index] {
                // println!("Splits at char: {:?}", current_str.as_bytes()[split_index]);
                split_index += 1;
            }

            let parent_rc = rc.borrow_mut().parent.clone().unwrap(); //reference to parent

            //initialize new internal node_____________________________________________________
            let new_internal_node = Node::new(config); //the split
            let new_internal_rc = Rc::new(RefCell::new(new_internal_node)); //ref to internal_node
            let new_internal_str = String::from(&current_str[0..split_index]);
            new_internal_rc.borrow_mut().depth = rc.borrow().depth;
            new_internal_rc.borrow_mut().string_index = (index , index + new_internal_str.len());
            new_internal_rc.borrow_mut().parent = Some(parent_rc.clone()); //set the split's parent to the current node's parent
            // println!("new internal node edge: {:?}", new_internal_rc.borrow().get_string(string));

            //initialize leaf_____________________________________________________
            let new_leaf_node = Node::new(config);
            let new_leaf_rc = Rc::new(RefCell::new(new_leaf_node)); //ref to leaf_node
            new_leaf_rc.borrow_mut().depth = rc.borrow().depth + 1;
            new_leaf_rc.borrow_mut().parent = Some(new_internal_rc.clone());
            new_leaf_rc.borrow_mut().string_index = (index + split_index, string.len());
            // println!("new leaf node edge: {:?}", new_leaf_rc.borrow().get_string(string));

            //update internal node parent to current parent_____________________________________________________
            rc.borrow_mut().parent = Some(new_internal_rc.clone());

            //update current node edge
            let new_indices = (rc.borrow().string_index.0 + new_internal_str.len(), rc.borrow().string_index.1);
            rc.borrow_mut().string_index = new_indices;
            rc.borrow_mut().depth += 1;
            // println!("new current node edge: {:?}", rc.borrow().get_string(string));

            //update the children
            new_internal_rc.borrow_mut().children.push(new_leaf_rc.clone()); //push the new leaf
            new_internal_rc.borrow_mut().children.push(rc.clone()); //push the current node
            Node::update_depth_recursive(rc.clone());
            Node::update_depth_recursive(new_leaf_rc.clone());

            //update the parent
            let index_of_cur_in_parent = parent_rc.borrow().children.clone().iter().position(|x| x.as_ptr() == rc.as_ptr()).unwrap();
            let mut vec_thingy = parent_rc.borrow().children.clone();
            vec_thingy.remove(index_of_cur_in_parent);
            vec_thingy.push(new_internal_rc.clone());
            parent_rc.borrow_mut().children = vec_thingy;

            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!RESULTS!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("Parent edge: {:?}", parent_rc.borrow().get_string(string));
            println!("Parent depth: {:?}", parent_rc.borrow().depth);
            println!("Parent count: {:?}", parent_rc.borrow().children.len());
            println!("Internal edge: {:?}", new_internal_rc.borrow().get_string(string));
            println!("Internal depth: {:?}", new_internal_rc.borrow().depth);
            println!("Internal count: {:?}", new_internal_rc.borrow().children.len());
            println!("Cur edge: {:?}", rc.borrow().get_string(string));
            println!("Cur depth: {:?}", rc.borrow().depth);
            println!("Cur count: {:?}", rc.borrow().children.len());
            println!("New edge: {:?}", new_leaf_rc.borrow().get_string(string));
            println!("New depth: {:?}", new_leaf_rc.borrow().depth);
            println!("New count: {:?}", new_leaf_rc.borrow().children.len());
            println!("---ENDRESULTS---");
        }
    }


    /// Return the node contaning the given string under the given node
    pub fn node_hops(mut rc: Rc<RefCell<Node>>, string: &str, alpha: &str) -> Option<Rc<RefCell<Node>>> {
        // remember that if you return a node, wrap it in a Some()
        // if no node is found, return None
        let mut target_string = String::from(alpha);
        let mut target_dollar = String::from(alpha) + "$";
        'outer: loop {
            println!("Current new substring: {:?}", target_string);
            let rc_children = rc.borrow().children.clone();
            for child in &rc_children {
                if target_dollar == child.borrow().get_string(string) {
                    println!("---Found the node. Has edge string of: {:?}", child.borrow().get_string(string));
                    return Some(child.clone()); //found the node we want, return it
                }
                if target_string.starts_with(&child.borrow().get_string(string)) { //found viable child
                    if target_string == child.borrow().get_string(string) {
                        println!("---Found the node. Has edge string of: {:?}", child.borrow().get_string(string));
                        return Some(child.clone()); //found the node we want, return it
                    }
                    //found a candidate, enter the child
                    println!("Located good candidate, edge of {:?}, depth {:?}, id {:?}",
                        child.borrow().get_string(string),
                        child.borrow().depth,
                        child.borrow().id,
                    );
                    rc = child.clone();
                    target_string = String::from(&target_string[child.borrow().get_string(string).len()..]);
                    target_dollar = String::from(&target_string) + "$";
                    continue 'outer; //restart the outer loop
                }
            }
            // will reach here if no node was found with the provided string
            println!("!!!!!!!!!!!!!!!!!!!!!!No node found!!!!!!!!!!!!!!!!!!!");
            
            return Some(rc);
        }
    }
}




