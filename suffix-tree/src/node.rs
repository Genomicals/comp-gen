
/*
Node:
    link to parent, option
    suffix link
    string index
*/

use std::{rc::Rc, cell::RefCell, collections::HashSet};



#[derive(Clone, Default, PartialEq, Debug)]
pub struct TreeConfig {
    pub next_id: usize,
    pub string: String,
    pub alphabet: HashSet<char>,
}
impl TreeConfig {
    pub fn new(string: &str, alphabet: HashSet<char>) -> Self {
        TreeConfig {next_id: 0, string: String::from(string), alphabet: alphabet}
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
    pub string_depth: usize,
}
impl Node {
    pub fn new(config: &mut TreeConfig) -> Self {
        Node {
            id: config.next(),
            parent: None,
            string_index: (0, 0),
            children: Vec::with_capacity(config.alphabet.len() + 1), //alphabet + $
            suffix_link: None,
            depth: 0,
            string_depth: 0,
        }
    }


    /// Prints the current node's info
    pub fn as_string(&self, config: &TreeConfig) -> String {
        format!("id: {:?}, depth: {:?}, string_depth: {:?}, num_children: {}, edge: {}\"{}\"{}, str_above: \"{}\"",
            self.id,
            self.depth,
            self.string_depth,
            self.children.len(),
            self.string_index.0,
            self.get_string(config),
            self.string_index.1,
            Node::reconstruct_string(self.parent.clone().unwrap(), config)
        )
    }


    // Reconstructs the string above the current node
    pub fn reconstruct_string(rc: Rc<RefCell<Node>>, config: &TreeConfig) -> String {
        if rc.borrow().string_index.1 == 0 {
            return String::new();
        }
        let self_str = String::from(rc.borrow().get_string(config));
        let parent_rc = rc.borrow().parent.clone().unwrap();
        let parent_result = Node::reconstruct_string(parent_rc, config);
        self_str + "|" + &parent_result
    }


    /// Depth-first print, for debugging
    pub fn print_tree(rc: Rc<RefCell<Node>>, config: &TreeConfig) {
        println!("Reached node, {}", rc.borrow().as_string(config));

        for child in rc.borrow().children.clone() {
            Node::print_tree(child, config);
            ////println!("Returned to node {:?}", rc.borrow().id);
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
    pub fn get_string(&self, config: &TreeConfig) -> String {
        String::from(&config.string[self.string_index.0..self.string_index.1])
    }


    /// Inserts the given suffix (of 'string' starting at 'index') under the given node
    pub fn find_path(rc: Rc<RefCell<Node>>, index: usize, config: &mut TreeConfig) -> Rc<RefCell<Node>> {
        println!(">>>Inserting new node under node: {}", rc.borrow().as_string(config));
        let target_str = String::from(&config.string[index..]); //the string we want to insert
        println!("String to add = {}", &target_str);

        // want to iterate through all children to find a good candidate
        let rc_children = rc.borrow().children.clone();
        for child in &rc_children {
            if child.borrow().get_string(config).as_bytes()[0] == target_str.as_bytes()[0] { //found a child to split or recurse to
                let child_str = child.borrow().get_string(config);
                let mut split_index = 0;
                while child_str.len() > split_index && target_str.len() > split_index && child_str.as_bytes()[split_index] == target_str.as_bytes()[split_index] { //want to find how far they match
                    split_index += 1;
                }
                
                // recurse down the child
                if split_index == child_str.len() {
                    drop(target_str);
                    drop(child_str);
                    return Node::find_path(child.clone(), index + split_index, config); //recursion
                }

                // can't recurse down the child, so split the child
                let new_internal_rc = Rc::new(RefCell::new(Node::new(config)));
                new_internal_rc.borrow_mut().string_index = (child.borrow().string_index.0, child.borrow().string_index.0 + split_index);
                new_internal_rc.borrow_mut().parent = Some(rc.clone());
                new_internal_rc.borrow_mut().children.push(child.clone()); //no need to sort, only one child
                new_internal_rc.borrow_mut().depth = rc.borrow().depth + 1;
                new_internal_rc.borrow_mut().string_depth = rc.borrow().string_depth + split_index;

                // update child
                let new_indices = (new_internal_rc.borrow().string_index.1, child.borrow().string_index.1);
                child.borrow_mut().string_index = new_indices;
                child.borrow_mut().depth += 1;

                // remove child from rc's children and push the new internal node
                let index_of_child_in_rc = rc.borrow().children.clone().iter().position(|x| x.as_ptr() == child.as_ptr()).unwrap();
                let mut new_children = rc.borrow().children.clone();
                new_children.remove(index_of_child_in_rc);
                new_children.push(new_internal_rc.clone());
                rc.borrow_mut().children = new_children;
                rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
                    if x.borrow().get_string(config) > y.borrow().get_string(config) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                Node::update_depth_recursive(child.clone()); //ensure the children have an increased depth by 1

                // create new leaf node
                let internal_len = new_internal_rc.borrow().string_index.1 - new_internal_rc.borrow().string_index.0;
                let mut new_leaf_node = Node::new(config);
                new_leaf_node.parent = Some(new_internal_rc.clone()); //set the parent
                new_leaf_node.depth = new_internal_rc.borrow().depth + 1;
                new_leaf_node.string_index = (index + internal_len, config.string.len()); //set the start index after the current node's length
                new_leaf_node.string_depth = new_internal_rc.borrow().string_depth + config.string.len() - new_leaf_node.string_index.0;
                let new_leaf_rc = Rc::new(RefCell::new(new_leaf_node));
                new_internal_rc.borrow_mut().children.push(new_leaf_rc.clone());
                new_internal_rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
                    if x.borrow().get_string(config) > y.borrow().get_string(config) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                
                println!("CASE: split the child and created a leaf");
                println!("Added split node: {}", new_internal_rc.borrow().as_string(config));
                println!("Added leaf node: {}", new_leaf_rc.borrow().as_string(config));
                return new_leaf_rc;
            } 
        }

        // didn't find a good child, so add a new one
        let mut new_node = Node::new(config);
        new_node.parent = Some(rc.clone()); //set the parent
        new_node.depth = rc.borrow().depth + 1;
        new_node.string_index = (index, config.string.len()); //set the start index after the current node's length
        new_node.string_depth = rc.borrow().string_depth + config.string.len() - new_node.string_index.0;
        let new_node_rc = Rc::new(RefCell::new(new_node));
        rc.borrow_mut().children.push(new_node_rc.clone());
        rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
            if x.borrow().get_string(config) > y.borrow().get_string(config) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        println!("CASE: created a leaf");
        println!("Added leaf node: {}", new_node_rc.borrow().as_string(config));
        return new_node_rc;
    }
    

    /// Return the node contaning the given string under the given node
    pub fn node_hops_pure(mut rc: Rc<RefCell<Node>>, alpha: &str, config: &TreeConfig) -> Option<Rc<RefCell<Node>>> {
        println!("==> Performing a node hop");
        let mut target_string = String::from(alpha);
        'outer: loop {
            println!("Current node: {}", rc.borrow().as_string(config));
            println!("Finding substring: {:?}", target_string);
            let rc_children: Vec<Rc<RefCell<Node>>> = rc.borrow().children.clone();
            for child in &rc_children {
                println!("Looking at child: {}", child.borrow().as_string(config));
                if target_string == child.borrow().get_string(config) {
                    println!("---Found the node. Has edge string of: {:?}", child.borrow().get_string(config));
                    return Some(child.clone()); //found the node we want, return it
                }
                if target_string.starts_with(&child.borrow().get_string(config)) { //found viable child
                    if target_string == child.borrow().get_string(config) {
                        println!("---Found the node. Has edge string of: {:?}", child.borrow().get_string(config));
                        return Some(child.clone()); //found the node we want, return it
                    }
                    //found a candidate, enter the child
                    rc = child.clone();
                    target_string = String::from(&target_string[child.borrow().get_string(config).len()..]);
                    continue 'outer; //restart the outer loop
                }
            }
            println!("!!!!!!!!!!!!!!!!!!!!!!No node found!!!!!!!!!!!!!!!!!!!");
            return None;
        }
    }


    /// Return the node contaning the given string under the given node, create an internal node if valid
    pub fn node_hops(mut rc: Rc<RefCell<Node>>, alpha: &str, config: &mut TreeConfig) -> Option<Rc<RefCell<Node>>> {
        println!("==> Performing a node hop");
        let mut target_string = String::from(alpha);
        'outer: loop {
            println!("Current node: {}", rc.borrow().as_string(config));
            println!("Finding substring: {:?}", target_string);
            if target_string.len() == 0 {
                println!("---Found the node, exhausted the string.");
                return Some(rc.clone());
            }
            let mut rc_children: Vec<Rc<RefCell<Node>>> = rc.borrow().children.clone();
            for child in &mut rc_children {
                println!("Looking at child: {}", child.borrow().as_string(config));
                if target_string == child.borrow().get_string(config) {
                    println!("---Found the node. Has edge string of: {:?}", child.borrow().get_string(config));
                    return Some(child.clone()); //found the node we want, return it
                }
                if target_string.starts_with(&child.borrow().get_string(config)) { //found viable child
                    if target_string == child.borrow().get_string(config) {
                        println!("---Found the node. Has edge string of: {:?}", child.borrow().get_string(config));
                        return Some(child.clone()); //found the node we want, return it
                    }
                    //found a candidate, enter the child
                    rc = child.clone();
                    target_string = String::from(&target_string[child.borrow().get_string(config).len()..]);
                    continue 'outer; //restart the outer loop
                }
                let child_string = child.borrow().get_string(config);
                if child_string.as_bytes()[0] == target_string.as_bytes()[0] { //see if we can split this child to create a valid internal node to return
                    //INSERT INTERNAL NODE
                    println!("INSERTING INTERNAL NODE");
                    println!("child children: {}", child.borrow().children.len());
                    println!("child id: {}", child.borrow().id);
                    
                    let mut split_index = 0;
                    //while child_string.as_bytes()[split_index] != b'$' && target_string.as_bytes()[split_index] != b'$' && child_string.as_bytes()[split_index] == target_string.as_bytes()[split_index] {
                    while child_string.len() > split_index && target_string.len() > split_index && child_string.as_bytes()[split_index] == target_string.as_bytes()[split_index] {
                        split_index += 1;
                    }

                    // initialize new internal node
                    let new_internal_rc = Rc::new(RefCell::new(Node::new(config)));
                    new_internal_rc.borrow_mut().string_index = (child.borrow().string_index.0, child.borrow().string_index.0 + split_index);
                    new_internal_rc.borrow_mut().parent = Some(rc.clone());
                    new_internal_rc.borrow_mut().children.push(child.clone()); //no need to sort, only one child
                    new_internal_rc.borrow_mut().depth = rc.borrow().depth + 1;
                    new_internal_rc.borrow_mut().string_depth = rc.borrow().string_depth + split_index;

                    // update child
                    let new_indices = (new_internal_rc.borrow().string_index.1, child.borrow().string_index.1);
                    child.borrow_mut().string_index = new_indices;
                    child.borrow_mut().depth += 1;

                    // remove child from rc's children and push the new internal node
                    let index_of_child_in_rc = rc.borrow().children.clone().iter().position(|x| x.as_ptr() == child.as_ptr()).unwrap();
                    let mut new_children = rc.borrow().children.clone();
                    new_children.remove(index_of_child_in_rc);
                    new_children.push(new_internal_rc.clone());
                    rc.borrow_mut().children = new_children;
                    rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
                        if x.borrow().get_string(config) > y.borrow().get_string(config) {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Less
                        }
                    });
                    Node::update_depth_recursive(child.clone()); //ensure the children have an increased depth by 1
                    println!("Added split node: {}", new_internal_rc.borrow().as_string(config));
                    return Some(new_internal_rc);
                }
            }
            println!("!!!!!!!!!!!!!!!!!!!!!!No node found!!!!!!!!!!!!!!!!!!!");
            return None;
        }
    }


    /// Insert the given suffix, provided the previous suffix
    pub fn suffix_link_insert(rc: Rc<RefCell<Node>>, index: usize, config: &mut TreeConfig) -> Rc<RefCell<Node>> {
        let u_rc_maybe = rc.clone().borrow().parent.clone();
        if let None = u_rc_maybe {
            return rc;
        }
        let u_rc = u_rc_maybe.unwrap();
        let suffix_link_maybe = u_rc.borrow().suffix_link.clone();
        if let Some(v_rc) = suffix_link_maybe {
            //SL(u) is known
            if u_rc.borrow().id != 0 {
                // the parent is not the root, CASE IA
                println!("Case IA");
                let string_depth = v_rc.borrow().string_depth;
                return Node::find_path(v_rc.clone(), index + string_depth, config);

            } else {
                // the parent is the root, CASE IB
                println!("Case IB");
                return Node::find_path(v_rc.clone(), index, config);
            }
        } else {
            //SL(u) is not known
            let u_prime_rc = u_rc.borrow().parent.clone().unwrap();
            let v_prime_rc_maybe = u_prime_rc.borrow().suffix_link.clone();
            let v_prime_rc = v_prime_rc_maybe.unwrap();
            let u_prime_id = u_prime_rc.borrow().id;
            let v_rc; //declare variable, will be assigned in if/else block
            if u_prime_id != 0 {
                // the grandparent is not the root, CASE IIA
                println!("Case IIA");
                let v_prime_end = v_prime_rc.borrow().string_index.1; //end of v'
                let beta_len = u_rc.borrow().string_index.1 - u_rc.borrow().string_index.0; //length of beta, string between u' and u
                println!(">> v_prime_end: {}", v_prime_end);
                println!(">> beta_len: {}", beta_len);
                v_rc = Node::node_hops(v_prime_rc.clone(), &String::from(&config.string)[v_prime_end..(v_prime_end + beta_len)], config).unwrap(); //from end of v' through beta
            } else {
                // the grandparent is the root, CASE IIB
                println!("Case IIB");
                // v_prime ends at 0, because it's the root
                let beta_len = u_rc.borrow().string_index.1 - u_rc.borrow().string_index.0; //length of beta, string between u' and u - 1, NOTE: beta_prime is one less than beta
                println!("beta_len: {}", beta_len);
                v_rc = Node::node_hops(v_prime_rc.clone(), &String::from(&config.string)[index..(index + beta_len - 1)], config).unwrap(); //from end of v' through beta
                println!("v_rc children: {}", v_rc.borrow().children.len());
                println!("v_rc id: {}", v_rc.borrow().id);
                println!("v_rc indices: {}, {}", v_rc.borrow().string_index.0, v_rc.borrow().string_index.1);
            }
            u_rc.borrow_mut().suffix_link = Some(v_rc.clone()); //establish link
            if u_rc.borrow().get_string(config)[1..] != v_rc.borrow().get_string(config) {
                panic!("bruh");
            }
            //let new_index = v_rc.borrow().string_index.1;
            //let new_index = index + rc_len;
            let new_index = rc.borrow().string_index.0; //new index should be the old rc's index
            return Node::find_path(v_rc.clone(), new_index, config); //insert string starting at v's ending index
        }
    } 
}




