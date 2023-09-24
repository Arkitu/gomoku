use std::collections::HashMap;
use crossterm::style::Stylize;

mod infinite;

pub const MAX_HEIGHT: usize = 5;
pub const MAX_WIDTH: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Color {
    White,
    Black,
    #[default]
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
    pub last_move: Option<(usize, usize)>
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(MAX_HEIGHT, MAX_WIDTH),
            turn: Color::White,
            last_move: None
        }
    }
    /// Check if the game is over and return the winner. A player win if he has a line of 5 stones.
    pub fn check_win_for_cell(&self, cell: Cell) -> bool {
        let mut counter = 0;

        for x in cell.x.saturating_sub(4)..=(cell.x + 4) {
            match self.board.get(x, cell.y) {
                Some(Color::None) => counter = 0,
                Some(c) => if c == &cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                },
                None => break
            }
        }

        counter = 0;
        for y in cell.y.saturating_sub(4)..=(cell.y + 4) {
            match self.board.get(cell.x, y) {
                Some(Color::None) => counter = 0,
                Some(c) => if c == &cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                },
                None => break
            }
        }

        counter = 0;
        for pos in (cell.x.saturating_sub(4)..=(cell.x + 4)).zip(cell.y.saturating_sub(4)..=(cell.y + 4)) {
            match self.board.get(pos.0, pos.1) {
                Some(Color::None) => counter = 0,
                Some(c) => if c == &cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                },
                None => break
            }
        }
        counter = 0;
        for pos in (cell.x.saturating_sub(4)..=(cell.x + 4)).zip((cell.y.saturating_sub(4)..=(cell.y + 4)).rev()) {
            match self.board.get(pos.0, pos.1) {
                Some(Color::None) => counter = 0,
                Some(c) => if c == &cell.color {
                    counter += 1;
                    if counter >= 5 {
                        return true
                    }
                } else {
                    counter = 0
                },
                None => break
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
            color: self.turn
        })
    }
    pub fn play_no_win_check(&mut self, pos: (usize, usize)) {
        *self.board.get_mut(pos.0, pos.1).unwrap() = self.turn;
        self.last_move = Some(pos);
        self.turn = self.turn.inverse();
    }
    /// Returns the winner if there is one or None
    pub fn play_unchecked(&mut self, pos: (usize, usize)) -> Color {
        *self.board.get_mut(pos.0, pos.1).unwrap() = self.turn;
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
    pub fn play(&mut self, pos: (usize, usize)) -> Result<Color, GameError> {
        if self.turn == Color::None {
            return Err(GameError::GameOver)
        }
        if self.board.get(pos.0, pos.1) != Some(&Color::None) {
            return Err(GameError::IllegalMove)
        }
        Ok(self.play_unchecked(pos))
    }
    pub fn play_moves(&mut self, moves: &[(usize, usize)]) -> Result<Color, GameError> {
        for m in moves {
            match self.play(*m) {
                Ok(Color::None) => (),
                Ok(winner) => return Ok(winner),
                Err(e) => return Err(e)
            }
        }
        Ok(Color::None)
    }
    pub fn unplay(&mut self, pos: (usize, usize)) {
        *self.board.get_mut(pos.0, pos.1).unwrap() = Color::None;
        self.turn = self.turn.inverse();
    }
    pub fn print(&self) {
        for row in self.board.iter_rows() {
            for c in row.map(|c| c.get_char().on_dark_grey()) {
                print!("{}", c);
            }
            println!()
        }
    }
}
