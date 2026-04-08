pub struct GameState {
    pub board: [ [u32; 4]; 4],
    score: u8
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board: [[0,0,0,2], [2,0,0,2], [0,4,8,2], [16,4,0,0]], score: 0}
    }
}
