use crate::{Config, NamedString, Cell};

/// Implements Needleman-Wunsch
fn needleman_wunsch(s1: String, s2: String, config: &Config) {
    let mut matrix: Vec<Vec<Cell>> = Vec::with_capacity(s1.len());
    for _ in 0..s1.len() {
        matrix.push(Vec::with_capacity(s2.len()));
    }
}


/// Implements Smith-Waterman
fn smith_waterman(s1: String, s2: String, config: &Config) {
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