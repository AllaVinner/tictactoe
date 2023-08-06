use std::borrow::BorrowMut;

use crate::{
    player::Player, 
    coordinate::Coordinate
};


pub struct Board {
    grid: Vec<Cell>,
    height: usize,
    width: usize
}

pub enum Cell {
    Empty,
    Occupied(Player)
}

pub enum MoverError {
    OutOfBounds,
    Occupied
}


impl Board {

    pub fn new(height: usize, width: usize) -> Board {
        Board {
            grid: vec![None; (height*width) as usize],
            height: height,
            width: width
        }   
    }

    fn coord_to_idx(&self, coord: Coordinate) -> usize {
        coord.row as usize * self.height  + coord.col as usize
    }
    
    fn print(&self) {
        let row_sep = format!("+{}", "-+".repeat(self.width));
        for row in self.grid.chunks(self.height) {
            println!("{}", row_sep);
            for cell in row {
                let cell_char = match cell {
                    Cell::Empty => ' ',
                    Cell::Occupied(p) => p.name
                };
                print!("|{}", cell_char);
            }
            println!("|")
        }
        println!("{}", row_sep);
    }
    

    pub fn coord_outside(&self, coord: Coordinate) -> bool {
        if coord.row < 0 || coord.col < 0 {
            return true;
        }
        if coord.row >= self.height as i32 || coord.col >= self.width as i32 {
            return true;
        }
        return false;
    }

    pub fn get_coord(&self, coord: Coordinate) -> Option<Cell> {
        if self.coord_outside(coord) {
            return None;
        }
        Some(self.grid[self.coord_to_idx(coord)])
    }

    pub fn is_coord_empty(&self, coord: Coordinate) -> Option<bool> {
        if self.coord_outside(coord) {
            return None;
        }
        match self.get_coord(coord) {
            Some(_) => Some(false),
            None => Some(true)
        }
    }

    pub fn make_move(&mut self, coord: Coordinate, player: Player) -> Result<(), MoverError>{
        if self.coord_outside(coord) {
            return Err(MoverError::OutOfBounds);
        }

        match self.get_coord(coord) {
            None => Err(MoverError::OutOfBounds),
            Some(c) => match c {
                Cell::Empty => {
                    self.grid[self.get_coord(coord)] = Some(player);
                    self.get_coord = Some(player);
                    Ok(())
                },
                Cell::Occupied(p) => Err(MoverError::Occupied)
            }
        }

        self.grid[coord.row][coord.col] = Some(player);
    }

    fn get(&self, pos: Pos) -> Option<Player> {
        self.grid[pos.row][pos.col]
    }

}







