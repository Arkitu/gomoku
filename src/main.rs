mod game;
use game::{Game, GameError};

#[tokio::main]
async fn main() -> Result<(), GameError> {
    let mut game = Game::new();
    let moves = [
        (-2, 0),
        (-2, 1),
        (-1, 0),
        (-1, 1),
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (2, 0)
    ];
    game.play_moves(&moves)?;
    game.print();
    Ok(())
}