use anyhow::Result;
use std::io::Write;

mod game;
use game::{Game, GameError, Color};

mod ai;
use ai::AI;

fn main() -> Result<()> {
    let mut game = Game::new();
    let ai = AI::new(Color::Black);
    loop {
        game.print();
        let mut m: Option<(isize, isize)> = None;
        while m == None {
            let mut input = String::new();
            print!("{} Enter move (x,y): ", game.turn.get_char());
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut input)?;
            m = match input.split_once(',') {
                None => {
                    println!("Invalid input. Try again");
                    continue
                },
                Some((x, y)) => Some((x.trim().parse()?, y.trim().parse()?))
            }
        }
        match game.play(m.unwrap()) {
            Ok(Color::None) => (),
            Ok(winner) => {
                game.print();
                println!("{} wins!", winner);
                break
            },
            Err(GameError::IllegalMove) => {
                println!("Illegal move. Try again");
                continue
            },
            Err(GameError::GameOver) => {
                println!("Game over.");
                break
            }
        }
        ai.play_move(&mut game);
    }
    Ok(())
}