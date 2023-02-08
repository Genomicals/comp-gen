use crate::{Config};


/// Makes up one cell of a table
#[derive(Debug)]
struct Cell {
    s_score: i32,
    d_score: i32,
    i_score: i32,   
}
impl Cell {
    fn new() -> Cell {
        Cell {
            s_score: 0,
            d_score: 0,
            i_score: 0,
        }
    }

    /// Super-optimized comparison maxxing algorithm
    fn score(&self) -> i32 {
        if self.d_score > self.i_score {
            if self.d_score > self.s_score {
                self.d_score
            } else {
                self.s_score
            }
        } else {
            if self.i_score > self.s_score {
                self.i_score
            } else {
                self.s_score
            }
        }
    }
}


/// Implements Needleman-Wunsch for global alignment
pub fn needleman_wunsch(s1: &str, s2: &str, config: &Config) {
    let mut matrix: Vec<Vec<Cell>> = Vec::with_capacity(s1.len() + 1);
    for _ in 0..s1.len()+1 {
        let mut lis: Vec<Cell> = Vec::with_capacity(s2.len() + 1);
        for _ in 0..s2.len()+1 {
            lis.push(Cell::new())
        }
        matrix.push(lis); //push the new list of cells to the matrix
    }

    //println!("First string: {:?}", s1);
    //println!("Second string: {:?}", s2);

    /*
   
    S(0, 0) = 0
    I(0, 0) = 0
    D(0, 0) = 0

    S(0, j) = -oo
    D(0, j) = -oo
    I(0, j) = h + jg
    
    S(i, 0) = -oo
    D(i, 0) = h + ig
    I(i, 0) = -oo

    S(i, j) = max   | S(i-1, j-1) | + S(ai, bj) //this means mismatch vs true match score
                    | D(i-1, j-1) |
                    | I(i-1, j-1) |
    
    ai = 
    bj = 


    D(i, j) = max   | D(i-1, j) + g
                    | S(i-1, j) + h + g
                    | I(i-1, j) + h + g
    
    I(i, j) = max   | I(i, j-1) + g
                    | S(i, j-1) + h + g
                    | D(i, j-1) + h + g

    i is y is s1
    j is x is s2

    */

    let real_min = std::i32::MIN - config.h - config.g;

    // setup corner
    matrix[0][0].s_score = 0;
    matrix[0][0].d_score = 0;
    matrix[0][0].i_score = 0;
    
    // setup left side
    for i in 1..s1.len()+1 {
        matrix[i][0].s_score = real_min;
        matrix[i][0].d_score = config.h + config.g * i as i32;
        matrix[i][0].i_score = real_min;
    }

    // setup top
    for j in 1..s2.len()+1 {
        matrix[0][j].s_score = real_min;
        matrix[0][j].d_score = real_min;
        matrix[0][j].i_score = config.h + config.g * j as i32;
    }

    // fill in the inside
    let mut s_score: i32;
    let mut d_score: i32;
    let mut i_score: i32;
    for i in 1..s1.len()+1 {
        for j in 1..s2.len()+1 {

            // first handle d_score
            d_score = matrix[i-1][j].d_score + config.g;
            s_score = matrix[i-1][j].s_score + config.h + config.g;
            i_score = matrix[i-1][j].i_score + config.h + config.g;
            if d_score > s_score && d_score > i_score {
                matrix[i][j].d_score = d_score;
            } else if s_score > d_score && s_score > i_score {
                matrix[i][j].d_score = s_score;
            } else {
                matrix[i][j].d_score = i_score;
            }

            // then handle i_score
            i_score = matrix[i-1][j].i_score + config.h;
            d_score = matrix[i-1][j].d_score + config.h + config.g;
            s_score = matrix[i-1][j].s_score + config.h + config.g;
            if d_score > s_score && d_score > i_score {
                matrix[i][j].i_score = d_score;
            } else if s_score > d_score && s_score > i_score {
                matrix[i][j].i_score = s_score;
            } else {
                matrix[i][j].i_score = i_score;
            }

            // finally handle s_score
            d_score = matrix[i-1][j-1].d_score;
            i_score = matrix[i-1][j-1].i_score;
            s_score = matrix[i-1][j-1].s_score;
            let match_score = if s1.chars().nth(i-1).unwrap() == s2.chars().nth(j-1).unwrap() {
                config.true_match
            } else {
                config.mismatch
            };
            if d_score > s_score && d_score > i_score {
                matrix[i][j].s_score = d_score + match_score;
            } else if s_score > d_score && s_score > i_score {
                matrix[i][j].s_score = s_score + match_score;
            } else {
                matrix[i][j].s_score = i_score + match_score;
            }


            /*
            
            cat chat

             c a t
            c
            h
            a
            t

            cat -> chat
            c _ a t
            c h a t
            
            */

        }
    }

    // start the retrace
    let mut s1_str: String = String::with_capacity(s1.len() + s2.len());
    let mut s2_str: String = String::with_capacity(s1.len() + s2.len());
    let mut ma_str: String = String::with_capacity(s1.len() + s2.len());
    let mut i: usize = s1.len();
    let mut j: usize = s2.len();
    //let mut matches: i32 = 0;
    //let mut mismatches: i32 = 0;
    //let mut opening_gaps: i32 = 0;
    //let mut gap_extensions: i32 = 0;
    
    while i != 0 || j != 0 {
        let up = matrix[i-1][j].score();
        let left = matrix[i][j-1].score();
        let diag = matrix[i-1][j-1].score();
        if up > left && up > diag {
            s1_str.push(s1.chars().nth(i-1).unwrap());
            s2_str.push('-');
            ma_str.push(' ');
            
            i -= 1;
        } else if left > up && left > diag {
            s2_str.push(s2.chars().nth(j-1).unwrap());
            s1_str.push('-');
            ma_str.push(' ');

            j -= 1;
        } else { //move diagonally
            s1_str.push(s1.chars().nth(i-1).unwrap());
            s2_str.push(s2.chars().nth(j-1).unwrap());
            if s1.chars().nth(i-1).unwrap() == s2.chars().nth(j-1).unwrap() { //if match
                ma_str.push('|');
                //matches += 1;
            } else {
                ma_str.push(' ');
                //mismatches += 1;
            }

            i -= 1;
            j -= 1;
        }
    }
    
    s1_str = s1_str.chars().rev().collect::<String>();
    ma_str = ma_str.chars().rev().collect::<String>();
    s2_str = s2_str.chars().rev().collect::<String>();
    
    // s1       1    AC
    // string2  2
    let mut s1_header = config.s1_name.clone() + "  ";
    let mut s2_header = config.s2_name.clone() + "  ";

    // pad the shorter string
    if s1_header.len() > s2_header.len() {
        s2_header += &" ".repeat(s1_header.len() - s2_header.len());
    } else {
        s1_header += &" ".repeat(s2_header.len() - s1_header.len());
    }

    let ma_header = " ".repeat(s1_header.len());

    let mut s1_num_len: usize; //length of the number, for padding reasons
    let mut s2_num_len: usize; //length of the number, for padding reasons
    let mut s1_chunk: &str; //60 chars
    let mut s2_chunk: &str;
    let mut ma_chunk: &str;
    let mut s1_counter = 0;
    let mut s2_counter = 0;
    for i in 0..s1_str.len()/60 {
        //print a row of 60
        //s1_counter += 60;
        //s2_counter += 60;
                                              
        s1_chunk = &s1_str[i*60..(i+1)*60]; //the chunk we're printing now
        s2_chunk = &s2_str[i*60..(i+1)*60];
        ma_chunk = &ma_str[i*60..(i+1)*60];

        s1_counter += 60 - s1_chunk.matches('-').count(); //update the current location
        s2_counter += 60 - s2_chunk.matches('-').count();

        s1_num_len = (s1_counter+1).to_string().len(); //stringify the current location
        s2_num_len = (s2_counter+1).to_string().len();

        println!("{}{}{}{}  {}", s1_header, s1_counter+1, " ".repeat(5-s1_num_len), s1_chunk, s1_counter);
        println!("{}{}{}", ma_header, " ".repeat(5), ma_chunk);
        println!("{}{}{}{}  {}", s2_header, s2_counter+1, " ".repeat(5-s2_num_len), s2_chunk, s2_counter);
        println!();

        if i == s1_str.len()/60 - 1 { //ie, in the final loop
            s1_chunk = &s1_str[(i+1)*60..]; //the chunk we're printing now
            s2_chunk = &s2_str[(i+1)*60..];
            ma_chunk = &ma_str[(i+1)*60..];

            s1_counter += s1_chunk.matches('-').count(); //update the current location
            s2_counter += s2_chunk.matches('-').count();

            s1_num_len = (s1_counter+1).to_string().len(); //stringify the current location
            s2_num_len = (s2_counter+1).to_string().len();

            println!("{}{}{}{}  {}", s1_header, s1_counter+1, " ".repeat(5-s1_num_len), s1_chunk, s1.len());
            println!("{}{}{}", ma_header, " ".repeat(5), ma_chunk);
            println!("{}{}{}{}  {}", s2_header, s2_counter+1, " ".repeat(5-s2_num_len), s2_chunk, s2.len());        }
    }


    //String leftovers = s1_str.len() % 60
        //print a row with leftovers



    //println!("{}", s1_str);
    //println!("{}", ma_str);
    //println!("{}", s2_str);

    let mut matches = 0;
    let mut mismatches = 0;
    let mut in_gap = false;
    let mut gap_start = 0;
    let mut gap_extension = 0;
    for i in 0..s1_str.len() {
        if ma_str.as_bytes()[i] == '|' as u8 { //found a match
            in_gap = false;
            matches += 1;
        } else { //not a match
            if s1_str.as_bytes()[i] == '-' as u8 || s2_str.as_bytes()[i] == '-' as u8 { //encountered a gap
                if !in_gap {
                    in_gap = true;
                    gap_start += 1;
                    gap_extension += 1;
                } else {
                    gap_extension += 1;
                }
            } else { //not a match or a gap, ie a mismatch
                in_gap = false;
                mismatches += 1;
            }
        }
    }

    //println!("{:?}", matrix);
    println!("\n\n");
    println!("Report:\n");
    println!("Global optimal score = {}\n", matrix[s1.len()][s2.len()].score());
    println!("Number of:  matches = {}, mismatches = {}, opening gaps = {}, gap extensions = {}\n", matches, mismatches, gap_start, gap_extension);
    println!("Identities = {}/{} ({}%), Gaps = {}/{} ({}%)",
        matches, s1_str.len(), (Into::<f64>::into(matches) / s1_str.len() as f64 * 100.0) as i32,
        gap_extension, s1_str.len(), (Into::<f64>::into(gap_extension) / s1_str.len() as f64 * 100.0) as i32);

}


/// Implements Smith-Waterman for local alignment
pub fn smith_waterman(s1: &str, s2: &str, config: &Config) {
    let mut matrix: Vec<Vec<Cell>> = Vec::with_capacity(s1.len());
    for _ in 0..s1.len() {
        matrix.push(Vec::with_capacity(s2.len()));
    }
}



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

// % identity = # of matches in alignment/alignment length


/*
[
[Cell { s_score: 0, d_score: 0, i_score: 0 }, Cell { s_score: -2147483642, d_score: -2147483642, i_score: -6 }, Cell { s_score: -2147483642, d_score: -2147483642, i_score: -7 }, Cell { s_score: -2147483642, d_score: -2147483642, i_score: -8 }, Cell { s_score: -2147483642, d_score: -2147483642, i_score: -9 }],
[Cell { s_score: -2147483642, d_score: -6, i_score: -2147483642 }, Cell { s_score: 0, d_score: -12, i_score: 1 }, Cell { s_score: 0, d_score: -13, i_score: -8 }, Cell { s_score: 0, d_score: -14, i_score: -6 }, Cell { s_score: 0, d_score: -15, i_score: -10 }],
[Cell { s_score: -2147483642, d_score: -7, i_score: -2147483642 }, Cell { s_score: 0, d_score: -5, i_score: -8 }, Cell { s_score: 0, d_score: -6, i_score: 2 }, Cell { s_score: 0, d_score: -6, i_score: -2 }, Cell { s_score: 0, d_score: -6, i_score: -2 }],
[Cell { s_score: -2147483642, d_score: -8, i_score: -2147483642 }, Cell { s_score: 0, d_score: -14, i_score: -6 }, Cell { s_score: 0, d_score: -4, i_score: -2 }, Cell { s_score: 0, d_score: -6, i_score: 3 }, Cell { s_score: 0, d_score: -6, i_score: -2 }],
[Cell { s_score: -2147483642, d_score: -9, i_score: -2147483642 }, Cell { s_score: 0, d_score: -6, i_score: -10 }, Cell { s_score: 0, d_score: -5, i_score: -2 }, Cell { s_score: 0, d_score: -3, i_score: -2 }, Cell { s_score: 0, d_score: -6, i_score: 4 }],
[Cell { s_score: -2147483642, d_score: -10, i_score: -2147483642 }, Cell { s_score: 0, d_score: -6, i_score: -11 }, Cell { s_score: 0, d_score: -8, i_score: -2 }, Cell { s_score: 0, d_score: -4, i_score: -2 }, Cell { s_score: 0, d_score: -2, i_score: -2 }]]


*/


