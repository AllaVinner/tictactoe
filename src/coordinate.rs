use std::ops::{Add, Neg};



#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32
}



impl Neg for Coordinate {
    type Output = Coordinate;

    fn neg(self) -> Self::Output {
        Coordinate{row: -self.row, col: -self.col}
    }
}

// impl Add<Coordinate> for Coordinate {
//     type Output = Coordinate;

//     fn add(self, rhs: Direction) -> Self::Output {
//         Coordinate{row: self.r}
//     }
// }



