//#[derive(Debug)]
//pub struct Matrix {
//    matrix: Vec<Vec<>>
//}


/// Makes up one cell of a table
#[derive(Debug)]
pub struct Cell {
    pub s_score: i32,
    pub d_score: i32,
    pub i_score: i32,   
}
impl Cell {
    pub fn new() -> Cell {
        Cell {
            s_score: 0,
            d_score: 0,
            i_score: 0,
        }
    }

    /// Super-optimized comparison maxxing algorithm
    pub fn score(&self) -> i32 {
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
