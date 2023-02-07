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
    
    while i != 0 || j != 0 {
        if matrix[i][j].d_score > matrix[i][j].i_score && matrix[i][j].d_score > matrix[i][j].s_score { //move up
            s1_str.push(s1.chars().nth(i-1).unwrap());
            s2_str.push('-');
            ma_str.push(' ');
            
            i -= 1;
        } else if matrix[i][j].i_score > matrix[i][j].d_score && matrix[i][j].i_score > matrix[i][j].s_score { //move left
            s2_str.push(s2.chars().nth(j-1).unwrap());
            s1_str.push('-');
            ma_str.push(' ');

            j -= 1;
        } else { //move diagonally
            s1_str.push(s1.chars().nth(i-1).unwrap());
            s2_str.push(s2.chars().nth(j-1).unwrap());
            if s1.chars().nth(i-1).unwrap() == s2.chars().nth(j-1).unwrap() {
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

    println!("{}", s1_str);
    println!("{}", ma_str);
    println!("{}", s2_str);

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


