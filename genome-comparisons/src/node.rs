use std::{rc::Rc, cell::RefCell, collections::HashSet};


#[derive(Clone, Default, PartialEq, Debug)]
pub struct TreeConfig {
    pub next_id: usize,
    pub strings: Vec<String>, //a copy of each string in the tree
    pub alphabet: HashSet<char>,
}
impl TreeConfig {
    pub fn new(string: &str, alphabet: HashSet<char>) -> Self {
        TreeConfig {next_id: 0, strings: vec![String::from(string)], alphabet: alphabet}
    }

    pub fn next(&mut self) -> usize {
        self.next_id += 1;
        self.next_id - 1
    }
}


/// Node for a general suffix tree
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Node { //string$, suffix_link -> tring$
    pub id: usize,
    pub parent: Option<Rc<RefCell<Node>>>, //points to immediate parent
    pub string_index: (usize, usize), //the coordinates of the string this node contains
    pub children: Vec<Rc<RefCell<Node>>>, //children of this node
    pub suffix_link: Option<Rc<RefCell<Node>>>, //points to the suffix link
    pub depth: u32,
    pub string_depth: usize,
    pub source_string: usize, //the string this node's label comes from
    pub node_color: isize, //what string this node falls under
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
            source_string: 0,
            node_color: 0,
        }
    }


    /// Prints the current node's info
    pub fn as_string(&self, config: &TreeConfig) -> String {
        format!("id: {:?}, depth: {:?}, string_depth: {:?}, num_children: {}, edge: {}\"{}\"{}, str_above: \"{}\", source: {}, color: {}",
            self.id,
            self.depth,
            self.string_depth,
            self.children.len(),
            self.string_index.0,
            //self.get_string(config),
            &config.strings[self.source_string][self.string_index.0..self.string_index.1],
            self.string_index.1,
            Node::reconstruct_string(self.parent.clone().unwrap(), config),
            self.source_string,
            self.node_color,
        )
    }


    /// Returns the edge label for this node
    pub fn get_edge_string(&self, config: &TreeConfig) -> String {
        String::from(&config.strings[self.source_string][self.string_index.0..self.string_index.1])
    }


    // Reconstructs the string above the current node
    pub fn reconstruct_string_separators(rc: Rc<RefCell<Node>>, config: &TreeConfig) -> String {
        //println!("<<-- Visited node: {}", rc.borrow().as_string(config));
        let parent = rc.borrow().parent.clone().unwrap();
        //println!("Parent node is: {}", parent.borrow().as_string(config));
        if rc.borrow().string_index.1 == 0 {
            return String::new();
        }
        let rc_indices = rc.borrow().string_index;
        let self_str = String::from(&config.strings[rc.borrow().source_string][rc_indices.0..rc_indices.1]);
        let parent_rc = rc.borrow().parent.clone().unwrap();
        let parent_result = Node::reconstruct_string_separators(parent_rc, config);
        parent_result + "|" + &self_str
    }


    // Reconstructs the string above the current node
    pub fn reconstruct_string(rc: Rc<RefCell<Node>>, config: &TreeConfig) -> String {
        if rc.borrow().string_index.1 == 0 {
            return String::new();
        }
        let rc_indices = rc.borrow().string_index;
        let self_str = String::from(&config.strings[rc.borrow().source_string][rc_indices.0..rc_indices.1]);
        let parent_rc = rc.borrow().parent.clone().unwrap();
        let parent_result = Node::reconstruct_string(parent_rc, config);
        parent_result + &self_str
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


    /// Inserts the given suffix (of 'string' starting at 'index') under the given node
    pub fn find_path(rc: Rc<RefCell<Node>>, index: usize, source_string: usize, config: &mut TreeConfig) -> Rc<RefCell<Node>> {
        let target_str = &config.strings[source_string][index..]; //the string we want to insert println!("String to add = {}", &target_str); want to iterate through all children to find a good candidate
        let rc_children = rc.borrow().children.clone();
        for child in &rc_children {
            let string_indices = child.borrow().string_index;
            //println!("{}", config.strings[child.borrow().source_string].len());
            //println!("{:?}, {}", &string_indices, &child.borrow().source_string);
            //println!("{}", config.strings[child.borrow().source_string][string_indices.0..string_indices.1].as_bytes()[0]);
            if target_str.len() == 0 {
                //println!("bad indices created");
            }
            //println!("{}", target_str.as_bytes()[0]);
            if config.strings[child.borrow().source_string][string_indices.0..string_indices.1].as_bytes()[0] == target_str.as_bytes()[0] { //found a child to split or recurse to
                let child_str = &config.strings[child.borrow().source_string][string_indices.0..string_indices.1];
                let mut split_index = 0;
                while child_str.len() > split_index && target_str.len() > split_index && child_str.as_bytes()[split_index] == target_str.as_bytes()[split_index] { //want to find how far they match
                    split_index += 1;
                }
                
                // check if we exhaust 
                if split_index == child_str.len() {
                    //if child_str.as_bytes()[split_index - 1] == '$' as u8 { //the node already exists, make node mixed if two+ strings end here
                    if target_str.len() == child_str.len() { //this node is a leaf, make leaf mixed if two+ strings end here
                        if child.borrow().source_string != source_string { //make sure the source string of the node isn't the same as the one we're adding
                            child.borrow_mut().node_color = -1;
                            return rc;
                        }
                    } else { //we've exhausted the child's string but not the target
                        drop(target_str);
                        drop(child_str);
                        return Node::find_path(child.clone(), index + split_index, source_string, config); //recursion down the child
                    }
                }

                // can't recurse down the child, so split the child
                let new_internal_rc = Rc::new(RefCell::new(Node::new(config)));
                //new_leaf_node.source_string = source_string;
                //new_leaf_node.node_color = source_string as isize;
                new_internal_rc.borrow_mut().source_string = child.borrow().source_string;
                new_internal_rc.borrow_mut().node_color = child.borrow().node_color;
                new_internal_rc.borrow_mut().string_index = (child.borrow().string_index.0, child.borrow().string_index.0 + split_index);
                if new_internal_rc.borrow().string_index.0 >= new_internal_rc.borrow().string_index.1 {
                    println!("created another set of bad indices here");
                }
                new_internal_rc.borrow_mut().parent = Some(rc.clone());
                new_internal_rc.borrow_mut().children.push(child.clone()); //no need to sort, only one child
                new_internal_rc.borrow_mut().depth = rc.borrow().depth + 1;
                new_internal_rc.borrow_mut().string_depth = rc.borrow().string_depth + split_index;

                // update child
                //let new_indices = (new_internal_rc.borrow().string_index.1, child.borrow().string_index.1);
                let new_indices = (child.borrow().string_index.0 + split_index, child.borrow().string_index.1);
                child.borrow_mut().string_index = new_indices;
                child.borrow_mut().depth += 1;
                child.borrow_mut().parent = Some(new_internal_rc.clone());

                // remove child from rc's children and push the new internal node
                let index_of_child_in_rc = rc.borrow().children.clone().iter().position(|x| x.as_ptr() == child.as_ptr()).unwrap();
                let mut new_children = rc.borrow().children.clone();
                new_children.remove(index_of_child_in_rc);
                new_children.push(new_internal_rc.clone());
                rc.borrow_mut().children = new_children;
                rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
                    let x_indices = x.borrow().string_index;
                    let y_indices = y.borrow().string_index;
                    if config.strings[x.borrow().source_string][x_indices.0..x_indices.1] > config.strings[y.borrow().source_string][y_indices.0..y_indices.1] {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                Node::update_depth_recursive(child.clone()); //ensure the children have an increased depth by 1

                // create new leaf node by splitting child
                let internal_len = new_internal_rc.borrow().string_index.1 - new_internal_rc.borrow().string_index.0;
                let mut new_leaf_node = Node::new(config);
                new_leaf_node.source_string = source_string;
                //if source_string == 1 {
                //    println!("inserted a 1");
                //}
                new_leaf_node.node_color = source_string as isize;
                new_leaf_node.parent = Some(new_internal_rc.clone()); //set the parent
                new_leaf_node.depth = new_internal_rc.borrow().depth + 1;
                new_leaf_node.string_index = (index + internal_len, config.strings[source_string].len()); //set the start index after the current node's length
                if new_leaf_node.string_index.0 >= new_leaf_node.string_index.1 {
                    println!("created invalid indices");
                }
                new_leaf_node.string_depth = new_internal_rc.borrow().string_depth + config.strings[source_string].len() - new_leaf_node.string_index.0; //HERE made potentially bad changes
                let new_leaf_rc = Rc::new(RefCell::new(new_leaf_node));
                new_internal_rc.borrow_mut().children.push(new_leaf_rc.clone());
                new_internal_rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
                    let x_indices = x.borrow().string_index;
                    let y_indices = y.borrow().string_index;
                    if config.strings[x.borrow().source_string][x_indices.0..x_indices.1] > config.strings[y.borrow().source_string][y_indices.0..y_indices.1] {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                
                return new_leaf_rc;
            } 
        }

        // didn't find a good child, so add a new one
        let mut new_node = Node::new(config);
        new_node.source_string = source_string;
        //if source_string == 1 {
        //    println!("inserted a 1");
        //}
        new_node.node_color = source_string as isize;
        new_node.parent = Some(rc.clone()); //set the parent
        new_node.depth = rc.borrow().depth + 1;
        new_node.string_index = (index, config.strings[source_string].len()); //set the start index after the current node's length
        if new_node.string_index.0 >= new_node.string_index.1 {
            println!("created invalid indices here too");
        }
        new_node.string_depth = rc.borrow().string_depth + config.strings[source_string].len() - new_node.string_index.0;
        let new_node_rc = Rc::new(RefCell::new(new_node));
        //let cur_source_string = rc.borrow().source_string;
        rc.borrow_mut().children.push(new_node_rc.clone());
        rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
            let x_indices = x.borrow().string_index;
            let y_indices = y.borrow().string_index;
            if config.strings[x.borrow().source_string][x_indices.0..x_indices.1] > config.strings[y.borrow().source_string][y_indices.0..y_indices.1] {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        return new_node_rc;
    }
    

    /// Return the node contaning the given string under the given node
    pub fn node_hops_pure(mut rc: Rc<RefCell<Node>>, alpha: &str, config: &TreeConfig) -> Option<Rc<RefCell<Node>>> {
        let mut target_string = String::from(alpha);
        'outer: loop {
            let rc_children: Vec<Rc<RefCell<Node>>> = rc.borrow().children.clone();
            for child in &rc_children {
                let child_indices = child.borrow().string_index;
                let child_str = &config.strings[child.borrow().source_string][child_indices.0..child_indices.1];
                if target_string == child_str {
                    return Some(child.clone()); //found the node we want, return it
                }
                if target_string.starts_with(child_str) { //found viable child
                    if target_string == child_str {
                        return Some(child.clone()); //found the node we want, return it
                    }
                    //found a candidate, enter the child
                    rc = child.clone();
                    target_string = String::from(&target_string[child_str.len()..]);
                    continue 'outer; //restart the outer loop
                }
            }
            println!("!!!!!!!!!!!!!!!!!!!!!!No node found!!!!!!!!!!!!!!!!!!!");
            return None;
        }
    }


    /// Return the node contaning the given string under the given node, create an internal node if valid
    pub fn node_hops(mut rc: Rc<RefCell<Node>>, alpha: (usize, usize), source_string: usize, config: &mut TreeConfig) -> Option<Rc<RefCell<Node>>> {
        let mut target_string = &config.strings[source_string][alpha.0..alpha.1];
        'outer: loop {
            if target_string.len() == 0 {
                return Some(rc.clone());
            }
            let mut rc_children: Vec<Rc<RefCell<Node>>> = rc.borrow().children.clone();
            for child in &mut rc_children {
                let child_indices = child.borrow().string_index;
                let child_str = &config.strings[child.borrow().source_string][child_indices.0..child_indices.1];
                //if target_string == child_str {
                //    return Some(child.clone()); //found the node we want, return it
                //}
                if target_string.starts_with(child_str) { //found viable child
                    if target_string == child_str {
                        return Some(child.clone()); //found the node we want, return it
                    }
                    //found a candidate, enter the child
                    rc = child.clone();
                    target_string = &target_string[child_str.len()..];
                    continue 'outer; //restart the outer loop
                }
                let child_string = child_str;
                if child_string.as_bytes()[0] == target_string.as_bytes()[0] { //see if we can split this child to create a valid internal node to return
                    
                    let mut split_index = 0;
                    while child_string.len() > split_index && target_string.len() > split_index && child_string.as_bytes()[split_index] == target_string.as_bytes()[split_index] {
                        split_index += 1;
                    }

                    // initialize new internal node
                    let new_internal_rc = Rc::new(RefCell::new(Node::new(config)));
                    new_internal_rc.borrow_mut().source_string = child.borrow().source_string;
                    new_internal_rc.borrow_mut().node_color = child.borrow().node_color;
                    new_internal_rc.borrow_mut().string_index = (child.borrow().string_index.0, child.borrow().string_index.0 + split_index);
                    if new_internal_rc.borrow().string_index.0 >= new_internal_rc.borrow().string_index.1 {
                        println!("mmmmm created bad indices");
                    }
                    new_internal_rc.borrow_mut().parent = Some(rc.clone());
                    new_internal_rc.borrow_mut().children.push(child.clone()); //no need to sort, only one child
                    new_internal_rc.borrow_mut().depth = rc.borrow().depth + 1;
                    new_internal_rc.borrow_mut().string_depth = rc.borrow().string_depth + split_index;

                    // update child
                    //let new_indices = (new_internal_rc.borrow().string_index.1, child.borrow().string_index.1);
                    let new_indices = (child.borrow().string_index.0 + split_index, child.borrow().string_index.1);
                    child.borrow_mut().string_index = new_indices;
                    if child.borrow().string_index.0 >= child.borrow().string_index.1 {
                        println!("created bad indices down here as well");
                    }
                    child.borrow_mut().depth += 1;
                    child.borrow_mut().parent = Some(new_internal_rc.clone());

                    // remove child from rc's children and push the new internal node
                    let index_of_child_in_rc = rc.borrow().children.clone().iter().position(|x| x.as_ptr() == child.as_ptr()).unwrap();
                    let mut new_children = rc.borrow().children.clone();
                    new_children.remove(index_of_child_in_rc);
                    new_children.push(new_internal_rc.clone());
                    rc.borrow_mut().children = new_children;
                    rc.borrow_mut().children.sort_by(|x, y| { //alphabetically sort the list of children
                        let x_indices = x.borrow().string_index;
                        let y_indices = y.borrow().string_index;
                        if config.strings[x.borrow().source_string][x_indices.0..x_indices.1] > config.strings[y.borrow().source_string][y_indices.0..y_indices.1] {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Less
                        }
                    });
                    Node::update_depth_recursive(child.clone()); //ensure the children have an increased depth by 1
                    return Some(new_internal_rc);
                }
            }
            println!("!!!!!!!!!!!!!!!!!!!!!!No node found!!!!!!!!!!!!!!!!!!!");
            return None;
        }
    }


    /// Insert the given suffix, provided the previous suffix
    pub fn suffix_link_insert(rc: Rc<RefCell<Node>>, index: usize, source_string: usize, config: &mut TreeConfig) -> Rc<RefCell<Node>> {
        //println!("inserting new");
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
                let string_depth = v_rc.borrow().string_depth;
                return Node::find_path(v_rc.clone(), index + string_depth, source_string, config);

            } else {
                // the parent is the root, CASE IB
                return Node::find_path(v_rc.clone(), index, source_string, config);
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
                let v_start = index + v_prime_rc.borrow().string_depth; //end of v'
                let beta_len = u_rc.borrow().string_index.1 - u_rc.borrow().string_index.0; //length of beta, string between u' and u
                v_rc = Node::node_hops(v_prime_rc.clone(), (v_start, v_start + beta_len), source_string, config).unwrap(); //from end of v' through beta
            } else {
                // the grandparent is the root, CASE IIB
                // v_prime ends at 0, because it's the root
                let beta_len = u_rc.borrow().string_index.1 - u_rc.borrow().string_index.0; //length of beta, string between u' and u - 1, NOTE: beta_prime is one less than beta
                v_rc = Node::node_hops(v_prime_rc.clone(), (index, index + beta_len - 1), source_string, config).unwrap(); //from end of v' through beta
            }
            u_rc.borrow_mut().suffix_link = Some(v_rc.clone()); //establish link
            let new_index = rc.borrow().string_index.0; //new index should be the old rc's index
            return Node::find_path(v_rc.clone(), new_index, source_string, config); //insert string starting at v's ending index
        }
    } 
}




