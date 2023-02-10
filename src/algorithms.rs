use crate::{Config};
use crate::structs::Cell;




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
        }
    }

    // start the retrace
    let mut s1_str: String = String::with_capacity(s1.len() + s2.len());
    let mut s2_str: String = String::with_capacity(s1.len() + s2.len());
    let mut ma_str: String = String::with_capacity(s1.len() + s2.len());
    let mut i: usize = s1.len();
    let mut j: usize = s2.len();

    println!("lengths of strings: {}, {}", s1.len(), s2.len());
    //println!("coordinate 0,0: {:?}", matrix[0][0]);
    
    while i != 0 || j != 0 {
        let up = if i > 0 { //if at edge of matrix
            matrix[i-1][j].score()
        } else {
            real_min
        };
        let left = if j > 0 {
            matrix[i][j-1].score()
        } else {
            real_min
        };
        let diag = if i > 0 && j > 0 {
            matrix[i-1][j-1].score()
        } else {
            real_min
        };

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
            } else {
                ma_str.push(' ');
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

    // print the retrace
    for i in 0..s1_str.len()/60 {
                                              
        s1_chunk = &s1_str[i*60..(i+1)*60]; //the chunk we're printing now
        s2_chunk = &s2_str[i*60..(i+1)*60];
        ma_chunk = &ma_str[i*60..(i+1)*60];

        s1_num_len = (s1_counter+1).to_string().len(); //stringify the current location
        s2_num_len = (s2_counter+1).to_string().len();
        
        let s1_counter_next = s1_counter + 60 - s1_chunk.matches('-').count(); //calculate the new location
        let s2_counter_next = s2_counter + 60 - s2_chunk.matches('-').count(); //calculate the new location

        println!("{}{}{}{}  {}", s1_header, s1_counter+1, " ".repeat(5-s1_num_len), s1_chunk, s1_counter_next);
        println!("{}{}{}", ma_header, " ".repeat(5), ma_chunk);
        println!("{}{}{}{}  {}", s2_header, s2_counter+1, " ".repeat(5-s2_num_len), s2_chunk, s2_counter_next);
        println!();

        s1_counter = s1_counter_next; //update the current location
        s2_counter = s2_counter_next;
    }
    let leftover = s1_str.len() % 60;
    if leftover > 0 { //have some left over
        s1_chunk = &s1_str[s1_str.len()-leftover..]; //the chunk we're printing now
        s2_chunk = &s2_str[s1_str.len()-leftover..];
        ma_chunk = &ma_str[s1_str.len()-leftover..];

        s1_num_len = (s1_counter+1).to_string().len(); //stringify the current location
        s2_num_len = (s2_counter+1).to_string().len();

        println!("{}{}{}{}  {}", s1_header, s1_counter+1, " ".repeat(5-s1_num_len), s1_chunk, s1.len());
        println!("{}{}{}", ma_header, " ".repeat(5), ma_chunk);
        println!("{}{}{}{}  {}", s2_header, s2_counter+1, " ".repeat(5-s2_num_len), s2_chunk, s2.len());
    }

    let mut matches = 0;
    let mut mismatches = 0;
    let mut in_gap = false;
    let mut gap_start = 0;
    let mut gap_extension = 0;
    
    // calculate matches, mismatches, gap openings and gap extensions
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
    let mut matrix: Vec<Vec<Cell>> = Vec::with_capacity(s1.len() + 1);
    for _ in 0..s1.len()+1 {
        let mut lis: Vec<Cell> = Vec::with_capacity(s2.len() + 1);
        for _ in 0..s2.len()+1 {
            lis.push(Cell::new())
        }
        matrix.push(lis); //push the new list of cells to the matrix
    }

    // setup corner
    matrix[0][0].s_score = 0;
    matrix[0][0].d_score = 0;
    matrix[0][0].i_score = 0;
    
    // setup left side
    for i in 1..s1.len()+1 {
        matrix[i][0].s_score = 0;
        matrix[i][0].d_score = 0;
        matrix[i][0].i_score = 0;
    }

    // setup top
    for j in 1..s2.len()+1 {
        matrix[0][j].s_score = 0;
        matrix[0][j].d_score = 0;
        matrix[0][j].i_score = 0;
    }

    // fill in the inside
    let mut s_score: i32;
    let mut d_score: i32;
    let mut i_score: i32;
    let mut top_i: usize = s1.len();
    let mut top_j: usize = s2.len();
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

            // fix all negative scores
            if matrix[i][j].d_score < 0 {
                matrix[i][j].d_score = 0;
            }
            if matrix[i][j].i_score < 0 {
                matrix[i][j].i_score = 0;
            }
            if matrix[i][j].s_score < 0 {
                matrix[i][j].s_score = 0;
            }

            // check to see if this cell is the highest scoring
            if matrix[i][j].score() > matrix[top_i][top_j].score() {
                top_i = i;
                top_j = j;
            }
        }
    }

    // start the retrace
    let mut s1_str: String = String::with_capacity(s1.len() + s2.len());
    let mut s2_str: String = String::with_capacity(s1.len() + s2.len());
    let mut ma_str: String = String::with_capacity(s1.len() + s2.len());
    let mut i: usize = top_i;
    let mut j: usize = top_j;

    while matrix[i][j].score() != 0 {
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
            } else {
                ma_str.push(' ');
            }

            i -= 1;
            j -= 1;
        }
    }
    let i_0 = i; //coordinates of the 0,0
    let j_0 = j;
    
    // print the output
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
    let mut s1_counter = i_0;
    let mut s2_counter = j_0;
    
    // print the retrace
    for i in 0..s1_str.len()/60 {
                                              
        s1_chunk = &s1_str[i*60..(i+1)*60]; //the chunk we're printing now
        s2_chunk = &s2_str[i*60..(i+1)*60];
        ma_chunk = &ma_str[i*60..(i+1)*60];

        s1_num_len = (s1_counter+1).to_string().len(); //stringify the current location
        s2_num_len = (s2_counter+1).to_string().len();
        
        let s1_counter_next = s1_counter + 60 - s1_chunk.matches('-').count(); //calculate the new location
        let s2_counter_next = s2_counter + 60 - s2_chunk.matches('-').count();

        println!("{}{}{}{}  {}", s1_header, s1_counter+1, " ".repeat(5-s1_num_len), s1_chunk, s1_counter_next);
        println!("{}{}{}", ma_header, " ".repeat(5), ma_chunk);
        println!("{}{}{}{}  {}", s2_header, s2_counter+1, " ".repeat(5-s2_num_len), s2_chunk, s2_counter_next);
        println!();

        s1_counter = s1_counter_next; //update the current location
        s2_counter = s2_counter_next;
    }
    let leftover = s1_str.len() % 60;
    if leftover > 0 { //have some left over
        s1_chunk = &s1_str[s1_str.len()-leftover..]; //the chunk we're printing now
        s2_chunk = &s2_str[s1_str.len()-leftover..];
        ma_chunk = &ma_str[s1_str.len()-leftover..];

        s1_num_len = (s1_counter+1).to_string().len(); //stringify the current location
        s2_num_len = (s2_counter+1).to_string().len();

        println!("{}{}{}{}  {}", s1_header, s1_counter+1, " ".repeat(5-s1_num_len), s1_chunk, s1_counter+leftover);
        println!("{}{}{}", ma_header, " ".repeat(5), ma_chunk);
        println!("{}{}{}{}  {}", s2_header, s2_counter+1, " ".repeat(5-s2_num_len), s2_chunk, s2_counter+leftover);
    }

    let mut matches = 0;
    let mut mismatches = 0;
    let mut in_gap = false;
    let mut gap_start = 0;
    let mut gap_extension = 0;

    // calculate matches, mismatches, gap openings and gap extensions
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

    println!("\n\n");
    println!("Report:\n");
    println!("Local optimal score = {}\n", matrix[top_i][top_j].score());
    println!("Number of:  matches = {}, mismatches = {}, opening gaps = {}, gap extensions = {}\n", matches, mismatches, gap_start, gap_extension);
    println!("Identities = {}/{} ({}%), Gaps = {}/{} ({}%)",
        matches, s1_str.len(), (Into::<f64>::into(matches) / s1_str.len() as f64 * 100.0) as i32,
        gap_extension, s1_str.len(), (Into::<f64>::into(gap_extension) / s1_str.len() as f64 * 100.0) as i32);
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


