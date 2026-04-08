use rand::prelude::*;

pub struct GameState {
    pub board: [ [u32; 4]; 4],
    score: u8
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board: init_board(), score: 0}
    }
}

fn init_board() -> [[u32; 4]; 4] {
    let mut board: [[u32; 4]; 4] = [[0; 4]; 4];
    let (row, col) = get_random_tile(&board);
    board[row][col] = if rand::random_range(0..100) < 90 { 2 } else { 4 };
    let (row, col) = get_random_tile(&board);
    board[row][col] = if rand::random_range(0..100) < 90 { 2 } else { 4 };
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
