use crate::game::{Game, Color, Board, Cell};

pub struct AI {
    pub color: Color
}
impl AI {
    pub fn new(color: Color) -> AI {
        AI {
            color
        }
    }
    /// More means better
    pub fn evaluate(&self, game: &Game, recursion_lvl: usize) -> isize {
        let mut score = 0;

        if recursion_lvl > 0 {
            AI::new(self.color.inverse()).
        }

        let mut line_color = Color::None;
        let mut counter: isize = 0;
        for x in game.board.min_x..game.board.max_x {
            for y in game.board.min_y..=(game.board.max_y+1) {
                let color = game.board.get(&(x, y));
                if color == line_color {
                    if color == Color::None {
                        continue
                    }
                    counter += 1
                } else {
                    let multiplier = if line_color == self.color {
                        1
                    } else if line_color == Color::None {
                        0
                    } else {
                        -1
                    };
                    score += counter.pow(2) * multiplier;
                    counter = 0;
                    line_color = color;
                }
            }
        }

        line_color = Color::None;
        counter = 0;
        for y in game.board.min_y..game.board.max_y {
            for x in game.board.min_x..=(game.board.max_x+1) {
                let color = game.board.get(&(x, y));
                if color == line_color {
                    if color == Color::None {
                        continue
                    }
                    counter += 1
                } else {
                    let multiplier = if line_color == self.color {
                        1
                    } else if line_color == Color::None {
                        0
                    } else {
                        -1
                    };
                    score += counter.pow(2) * multiplier;
                    counter = 0;
                    line_color = color;
                }
            }
        }

        score
    }
    pub fn get_best_move(&self, game: &Game, recursion_lvl: usize) -> (isize, isize) {
        if game.turn != self.color {
            panic!("Playing turn whereas its not ai's turn")
        }
        let mut game = (*game).clone();
        let min_x = game.board.min_x;
        let max_x = game.board.max_x;
        let min_y = game.board.min_y;
        let max_y = game.board.max_y;

        let mut best_move = (0, 0);
        let mut best_score = isize::MIN;
        for x in (game.board.min_x-2)..(game.board.max_x+2) {
            for y in (game.board.min_y-2)..(game.board.max_x+2) {
                let pos = (x, y);
                if game.board.get(&pos) != Color::None {
                    continue
                }
                game.play_no_win_check(pos);
                if game.check_win_for_cell(Cell {
                    color: self.color,
                    x: pos.0,
                    y: pos.1
                }) {
                    best_move = pos;
                    best_score = isize::MAX;
                }
                let score = self.evaluate(&game, recursion_lvl);
                if score > best_score {
                    best_move = pos;
                    best_score = score;
                }
                game.unplay(pos);
                game.board.min_x = min_x;
                game.board.max_x = max_x;
                game.board.min_y = min_y;
                game.board.max_y = max_y;
            }
        }
        best_move
    }
    pub fn play_move(&self, game: &mut Game) {
        game.play(self.get_best_move(&game, 3)).unwrap();
    }
}