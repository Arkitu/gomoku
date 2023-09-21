use std::collections::HashMap;
use crossterm::style::Stylize;

mod infinite;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
    None
}
impl Color {
    pub fn inverse(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
            Self::None => Self::None
        }
    }
    pub fn get_char(&self) -> char {
        match self {
            Self::White => '●',
            Self::Black => '○',
            Self::None => '•'
        }
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "White"),
            Self::Black => write!(f, "Black"),
            Self::None => write!(f, "None")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Cell {
    pub color: Color,
    pub x: usize,
    pub y: usize
}

type Board = grid::Grid<Color>;

#[derive(Debug)]
pub enum GameError {
    IllegalMove,
    GameOver
}
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::IllegalMove => write!(f, "Illegal move"),
            GameError::GameOver => write!(f, "Game over")
        }
    }
}
impl std::error::Error for GameError {}

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub last_move: Option<(isize, isize)>
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(100, 100),
            turn: Color::White,
            last_move: None
        }
    }
    /// Check if the game is over and return the winner. A player win if he has a line of 5 stones.
    pub fn check_win_for_cell(&self, cell: Cell) -> bool {
        let mut counter = 0;
        let line = self.board.iter_row(cell.y).[cell.x.saturating_sub(4)]
        for x in (cell.x - 4).max(self.board.min_x)..=(cell.x + 4).min(self.board.max_x) {
            match self.board.get(x, cell.y).unwrap() {
                Color::None => counter = 0,
                c => if c == cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                }
            }
        }
        counter = 0;
        for y in (cell.y - 4).max(self.board.min_y)..=(cell.y + 4).min(self.board.max_y) {
            match self.board.get(&(cell.x, y)) {
                Color::None => counter = 0,
                c => if c == cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                }
            }
        }
        counter = 0;
        for pos in ((cell.x - 4).max(self.board.min_x)..=(cell.x + 4).min(self.board.max_x)).zip((cell.y - 4).max(self.board.min_y)..=(cell.y + 4).min(self.board.max_y)) {
            match self.board.get(&pos) {
                Color::None => counter = 0,
                c => if c == cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                }
            }
        }
        counter = 0;
        for pos in ((cell.x - 4).max(self.board.min_x)..=(cell.x + 4).min(self.board.max_x)).zip(((cell.y - 4).max(self.board.min_y)..=(cell.y + 4).min(self.board.max_y)).rev()) {
            match self.board.get(&pos) {
                Color::None => counter = 0,
                c => if c == cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                }
            }
        }
        false
    }
    /// Check win for last move
    pub fn check_win(&self) -> bool {
        let last_move = match self.last_move {
            Some(lm) => lm,
            None => return false
        };
        self.check_win_for_cell(Cell {
            x: last_move.0,
            y: last_move.1,
            color: self.turn.inverse()
        })
    }
    pub fn play_no_win_check(&mut self, pos: (isize, isize)) {
        self.board.set(pos, self.turn);
        self.last_move = Some(pos);
        self.turn = self.turn.inverse();
    }
    /// Returns the winner if there is one or None
    pub fn play_unchecked(&mut self, pos: (isize, isize)) -> Color {
        self.board.set(pos, self.turn);
        self.last_move = Some(pos);
        if self.check_win() {
            let winner = self.turn.clone();
            self.turn = Color::None;
            winner
        } else {
            self.turn = self.turn.inverse();
            Color::None
        }
    }
    /// Return Error if the move is illegal
    pub fn play(&mut self, pos: (isize, isize)) -> Result<Color, GameError> {
        if self.turn == Color::None {
            return Err(GameError::GameOver)
        }
        if self.board.get(&pos) != Color::None {
            return Err(GameError::IllegalMove)
        }
        Ok(self.play_unchecked(pos))
    }
    pub fn play_moves(&mut self, moves: &[(isize, isize)]) -> Result<Color, GameError> {
        for m in moves {
            match self.play(*m) {
                Ok(Color::None) => (),
                Ok(winner) => return Ok(winner),
                Err(e) => return Err(e)
            }
        }
        Ok(Color::None)
    }
    pub fn unplay(&mut self, pos: (isize, isize)) {
        self.board.set(pos, Color::None);
        self.turn = self.turn.inverse();
    }
    pub fn print(&self) {
        for y in self.board.min_y..=self.board.max_y {
            for x in self.board.min_x..=self.board.max_x {
                print!("{}", self.board.get(&(x, y)).get_char().on_dark_grey());
            }
            println!();
        }
    }
}
