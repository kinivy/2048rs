use rand::prelude::*;

pub struct GameState {
    pub board: [ [u32; 4]; 4],
    score: u32
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board: init_board(), score: 0}
    }

    pub fn shift_left(&mut self) {
       for i in 0..4 {
           let mut row = self.board[i];
           for j in 0..4 {
               for k in j+1..4 {
                   if row[k] != 0 {
                       if row[j] == row[k] {
                           row[j] *= 2;
                           row[k] = 0;
                       } else if row[j] == 0 {
                           row[j] = row[k];
                           row[k] = 0;
                       }
                   }
               }
           }
           self.board[i] = row;
       } 
    }

    pub fn shift_right(&mut self) {
       for i in 0..4 {
           let mut row = self.board[i];
           for j in (0..4).rev() {
               for k in (0..j).rev() {
                   if row[k] != 0 {
                       if row[j] == row[k] {
                           row[j] *= 2;
                           row[k] = 0;
                       } else if row[j] == 0 {
                           row[j] = row[k];
                           row[k] = 0;
                       }
                   }
               }
           }
           self.board[i] = row;
       } 
    }

    pub fn add_tile(&mut self) {
        let (row, col) = get_random_tile(&self.board);
        self.board[row][col] = new_tile_value();
    }
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        Self { board: self.board, score: self.score }
    }
}

fn init_board() -> [[u32; 4]; 4] {
    let mut board: [[u32; 4]; 4] = [[0; 4]; 4];
    for _ in 0..2 {
        let (row, col) = get_random_tile(&board);
        board[row][col] = new_tile_value();
    }
    board
}


fn get_random_tile(board: &[[u32; 4]; 4]) -> (usize, usize) {
    let row = rand::random_range(0..4);
    let col = rand::random_range(0..4);
    if board[row][col] != 0 {
        get_random_tile(board)
    } else {
        (row, col)
    }
} 

fn new_tile_value() -> u32 {
    if rand::random_range(0..100) < 90 { 2 } else { 4 }
}
