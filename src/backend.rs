use std::ops::{Add, Neg};

use text_io::read;


#[derive(Clone, Copy, Debug)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(Clone, Copy, Debug)]
struct Direction {
    row: isize,
    col: isize
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Player {
    name: char
}

struct Board {
    grid: Vec<Vec<Option<Player>>>,
    shape: Pos
}

struct TicTacToe {
    players: Vec<Player>,
    board: Board,
    ntic: usize,
}


fn create_empty_board(shape: Pos) -> Board {
    Board {
        grid: vec![vec![None; shape.col]; shape.row],
        shape: shape
    }
}

impl Board {
    fn print(&self) {
        let row_sep = format!("+{}", "-+".repeat(self.shape.col));
        for row in &self.grid {
            println!("{}", row_sep);
            for cell in row {
                let cell_char = match cell {
                    None => ' ',
                    Some(p) => p.name
                };
                print!("|{}", cell_char);
            }
            println!("|")
        }
        println!("{}", row_sep);
    }
    

    fn pos_outside(&self, pos: Pos) -> bool {
        if pos.row >= self.shape.row || pos.col >= self.shape.col {
            return true;
        }
        return false;
    }

    fn is_pos_empty(&self, pos: Pos) -> bool {
        match self.grid[pos.row][pos.col] {
            Some(_) => false,
            None => true
        }
    }

    fn make_move(&mut self, pos: Pos, player: Player){
        self.grid[pos.row][pos.col] = Some(player);
    }

    fn get(&self, pos: Pos) -> Option<Player> {
        self.grid[pos.row][pos.col]
    }

}


impl TicTacToe {

    fn play(&mut self) {
        for player in self.players.iter().cycle() {
            let m = self.get_move(*player);
            self.board.make_move(m, *player);
            self.board.print();
            if self.was_move_winning(m) {
                println!("Player {} won!", player.name);
                break;
            }
        }
    }

    fn get_move(&self, player: Player) -> Pos {
        loop {
            print!("Player {} turn: ", player.name);
            let row = read!();
            let col = read!();
            let pos = Pos{row, col};
            if self.board.pos_outside(pos) {
                println!("In put was outside of the board.");
                continue;
            }
            if ! self.board.is_pos_empty(pos) {
                println!("Input position already occupied.");
                continue;
            }
            return pos
        }
    }

    fn was_move_winning(&self, m: Pos) -> bool {
        let player = match self.board.get(m) {
            None => return false,
            Some(p) => p
        };
        for direction in vec![
            Direction{row: 1, col: 0},
            Direction{row: 0, col: 1},
            Direction{row: 1, col: 1},
            Direction{row: 1, col: -1},
        ].into_iter() {
            let mut current_ntic = 1;
            for branch in vec![direction, -direction].into_iter(){
                let mut currsor = m;
                for _ in 1..self.ntic {
                    currsor = match currsor + branch {
                        None => break,
                        Some(c) => c
                    };

                    if self.board.pos_outside(currsor) {
                        break;
                    }
                    match self.board.get(currsor) {
                        None => break,
                        Some(p) => { 
                            if p == player {
                                current_ntic += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            if current_ntic >= self.ntic {
                return true;
            }
        }
        return false;
    }
}

impl Neg for Direction{
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction{row: -self.row, col: -self.col}
    }
}

impl Add<Direction> for Pos {
    type Output = Option<Pos>;

    fn add(self, rhs: Direction) -> Self::Output {
        let row = match self.row.checked_add_signed(rhs.row) {
            None => return None,
            Some(r) => r
        };
        let col = match self.col.checked_add_signed(rhs.col) {
            None => return None,
            Some(c) => c
        };
        Some(Pos{row, col})
    }
}

fn configure_tictactoe() -> TicTacToe {
    println!("Who will play?");
    let user_input: String = read!();
    let players: Vec<Player> = user_input.chars().map(|c| Player{name: c}).collect();
    println!("How big shall the board be? ");
    let row: usize = read!();
    let col: usize = read!();
    let board_shape = Pos{row, col};
    println!("How many in a row to win? ");
    let ntic: usize = read!();
    TicTacToe{
        players: players,
        board: create_empty_board(board_shape),
        ntic: ntic,
    }
}

fn main() {
    println!("Hello, world!");
    let mut game = configure_tictactoe();
    game.play();
    
}
