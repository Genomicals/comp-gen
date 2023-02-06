mod algorithms;

use std::fs;
use clap::{arg, command};

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


/// Used to keep config settings in one place
#[derive(Debug)]
struct Config {
    true_match: i32,
    mismatch: i32,
    h: i32,
    g: i32,
}


/// Makes up one cell of a table
#[derive(Debug)]
struct Cell {
    s_score: i32,
    d_score: i32,
    i_score: i32    
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
            -c --config <FILE> "Sets a custom config file"
        ))
        .arg(arg!(
            --needleman "Run the Needleman-Wunsch algorithm"
        ))
        .arg(arg!(
            --smith "Run the Smith-Waterman algorithm"
        ))
        .arg(arg!(
            [FILE] "Strings to operate on"
        ))
        .get_matches(); // run clap

    // Initialize default config settings
    let mut config = Config {
        true_match: 1,
        mismatch: -2,
        h: -5,
        g: -1,
    };

    // Read the config file
    let parameters_file: &str;
    match args.get_one::<String>("config") { //grab either the provided config or the default
        None => {
            parameters_file = "parameters.config";
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
    println!("Config file: {:?}", config);
    
    // Read the strings file
    let strings_file: &str;
    match args.get_one::<String>("FILE") { //grab either the provided config or the default
        None => {
            panic!("Could not retrieve a string file");
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

    println!("Input strings:");
    println!("**************");
    println!();
    for str in string_vec {
        println!("{:?}", str);
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
