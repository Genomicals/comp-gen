mod node;
mod api;

use std::{rc::Rc, cell::RefCell, fs, collections::HashSet};
use node::{Node, TreeConfig};
use api::Interface;
use clap::{arg, command};


/// Used to record the name of a string
#[derive(Debug)]
struct NamedString {
    name: String,
    string: String,
}
impl NamedString {
    fn name(name: &str) -> Self {
        NamedString { name: String::from(name), string: String::new() }
    }
}




/// Main entry point
fn main() {

    // Process the arguments with clap
    let args: clap::ArgMatches = command!()
        .arg(arg!(
            [FILE] "File containing the strings to compare"
        ))
        .get_matches(); // run clap

    //// Initialize default config settings
    //let mut config = Config {
    //    true_match: 1,
    //    mismatch: -2,
    //    h: -5,
    //    g: -1,
    //    s1_name: String::new(),
    //    s2_name: String::new(),
    //};

    //// Read the config file
    //let parameters_file: &str;
    //match args.get_one::<String>("CONFIG") { //grab either the provided config or the default
    //    None => {
    //        parameters_file = "configs/parameters.config";
    //    },
    //    Some(file) => {
    //        parameters_file = &file;
    //    }
    //}
    //let parameters = read_file(parameters_file);
    //let config_lines = parameters.lines();
    //for line in config_lines {
    //    let words: Vec<&str> = line.split_ascii_whitespace().collect();
    //    match words[0] {
    //        "match" => {
    //            config.true_match = words[1].parse().expect("Error parsing a match score.");
    //        },
    //        "mismatch" => {
    //            config.mismatch = words[1].parse().expect("Error parsing a mismatch score.");
    //        },
    //        "h" => {
    //            config.h = words[1].parse().expect("Error parsing an h score.");
    //        },
    //        "g" => {
    //            config.g = words[1].parse().expect("Error parsing a g score.");
    //        },
    //        _ => {
    //            println!("found something else idk man");
    //        }
    //    }
    //}
    
    // Read the strings file
    let strings_file: &str;
    match args.get_one::<String>("FILE") { //grab either the provided config or the default
        None => {
            panic!("Missing required command-line option: FILE");
        },
        Some(file) => {
            strings_file = &file;
        }
    }
    let file_as_str = fs::read_to_string(strings_file).expect("Error reading file");
    let file_lines = file_as_str.lines();
    let mut string_vec: Vec<NamedString> = Vec::new();
    for line in file_lines {
        if line.starts_with(">") { // skip this line but push a new string
            string_vec.push(NamedString::name(&line[1..line.len()])); //copy from the 1st index to the end
            continue;
        } else {
            string_vec.last_mut().expect("Input strings were in the wrong format").string.push_str(line);
        }
    }

    println!("well, all the formatting was done correctly");
    //config.s1_name = string_vec[0].name.clone();
    //config.s2_name = string_vec[1].name.clone();


}
