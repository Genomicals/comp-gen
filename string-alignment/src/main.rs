mod algorithms;
mod structs;

use std::fs;
use clap::{arg, command};
use algorithms::{needleman_wunsch, smith_waterman};
use structs::Config;

/// Used to record the name of a string
#[derive(Debug)]
struct NamedString {
    name: String,
    str: String,
}
impl NamedString {
    fn name(name: &str) -> Self {
        NamedString { name: String::from(name), str: String::new() }
    }
}


/// Reads a file and returns it as a string
fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Error reading file")
}


/// Main entry point
fn main() {

    // Process the arguments with clap
    let args: clap::ArgMatches = command!()
        .arg(arg!(
            [FILE] "File containing the strings to compare"
        ))
        .arg(arg!(
            [ALG] "0: Global (Needleman-Wunsch), 1: Local (Smith-Waterman)"
        ))
        .arg(arg!(
            [CONFIG] "Path to custom config file"
        ))
        .get_matches(); // run clap

    // Initialize default config settings
    let mut config = Config {
        true_match: 1,
        mismatch: -2,
        h: -5,
        g: -1,
        s1_name: String::new(),
        s2_name: String::new(),
    };

    // Read the config file
    let parameters_file: &str;
    match args.get_one::<String>("CONFIG") { //grab either the provided config or the default
        None => {
            parameters_file = "configs/parameters.config";
        },
        Some(file) => {
            parameters_file = &file;
        }
    }
    let parameters = read_file(parameters_file);
    let config_lines = parameters.lines();
    for line in config_lines {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        match words[0] {
            "match" => {
                config.true_match = words[1].parse().expect("Error parsing a match score.");
            },
            "mismatch" => {
                config.mismatch = words[1].parse().expect("Error parsing a mismatch score.");
            },
            "h" => {
                config.h = words[1].parse().expect("Error parsing an h score.");
            },
            "g" => {
                config.g = words[1].parse().expect("Error parsing a g score.");
            },
            _ => {
                println!("found something else idk man");
            }
        }
    }
    
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
    let strings = read_file(strings_file);
    let string_lines = strings.lines();
    let mut string_vec: Vec<NamedString> = Vec::new();
    for line in string_lines {
        if line.starts_with(">") { // skip this line but push a new string
            string_vec.push(NamedString::name(&line[1..line.len()])); //copy from the 1st index to the end
            continue;
        } else {
            string_vec.last_mut().expect("Input strings were in the wrong format").str.push_str(line);
        }
    }

    config.s1_name = string_vec[0].name.clone();
    config.s2_name = string_vec[1].name.clone();
    println!("OUTPUT:");
    println!("********\n");
    println!("Scores:    match = {}, mismatch = {}, h = {}, g = {}", config.true_match, config.mismatch, config.h, config.g);
    println!();
    println!("Sequence 1 = \"{}\", length = {} characters", string_vec[0].name, string_vec[0].str.len());
    println!("Sequence 2 = \"{}\", length = {} characters", string_vec[1].name, string_vec[1].str.len());
    println!();

    match args.get_one::<String>("ALG").unwrap().parse::<i32>() {
        Err(_) => {
            panic!("Missing required command-line option: ALG");
        },
        Ok(0) => { // run needleman-wunsch
            needleman_wunsch(&string_vec[0].str, &string_vec[1].str, &config);
        },
        Ok(1) => { // run smith-waterman
            smith_waterman(&string_vec[0].str, &string_vec[1].str, &config);
        },
        _ => {
            panic!("Invalid input for required command-line option: ALG");
        }
    }
}



// ATGGCGT
// ATG-AGT   global optimal score: +match -mismatch 0space

// struct DP_cell {
//     int Sscore;       // Substitution (S) score       
//     int Dscore;      // Deletion (D) score
//     int Iscore;        // Insertion (I) score 
//     ... // add any other field(s) that you may need for the implementation
// }





// INPUT:
// ******
// >s1
// ACATGCTACACGTATCCGATACCCCGTAACCGATAACGATACACAGACCTCGTACGCTTG
// CTACAACGTACTCTATAACCGAGAACGATTGACATGCCTCGTACACATGCTACACGTACT
// CCGAT
// >s2
// ACATGCGACACTACTCCGATACCCCGTAACCGATAACGATACAGAGACCTCGTACGCTTG
// CTAATAACCGAGAACGATTGACATTCCTCGTACAGCTACACGTACT
// CCGAT
// OUTPUT:
// ********
// Scores:    match = 1, mismatch = -2, h =-5, g = -2
// Sequence 1 = "s1", length = 125 characters
// Sequence 2 = "s2", length = 111 characters
// s1  1    ACATGCTACACGTATCCGATACCCCGTAACCGATAACGATACACAGACCTCGTACGCTTG  60
// |||||| ||||   ||||||||||||||||||||||||||||| ||||||||||||||||
// s2  1    ACATGCGACACTACTCCGATACCCCGTAACCGATAACGATACAGAGACCTCGTACGCTTG  60
// s1  61   CTACAACGTACTCTATAACCGAGAACGATTGACATGCCTCGTACACATGCTACACGTACT  120
// |||           ||||||||||||||||||||| |||||||||   ||||||||||||
// s2  61   CTA-----------ATAACCGAGAACGATTGACATTCCTCGTACA---GCTACACGTACT  106
// s1  121  CCGAT  125
// |||||
// s2  107  CCGAT  111
// Report:
// Global optimal score = 55
// Number of:  matches = 105, mismatches = 6, opening gaps = 2, gap 
// extensions = 14
// Identities = 105/125 (84%), Gaps = 14/125 (11%)

