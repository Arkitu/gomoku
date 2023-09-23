use neat_gru::neural_network::NeuralNetwork;

use crate::game::{Game, Color, Cell, MAX_WIDTH, MAX_HEIGHT, GameError};

struct Simulation {
    nets: Vec<NeuralNetwork<f64>>
}
impl Simulation {
    pub fn new() -> Self {
        Self {
            nets: Vec::new()
        }
    }
}
impl neat_gru::game::Game<f64> for Simulation {
    fn run_generation(&mut self) -> Vec<f64> {
        let scores: Vec<f64> = Vec::new();
        for [net1, net2] in self.nets.chunks_exact_mut(2) {
            let mut game = Game::new();
            let mut winner = Color::None;
            loop {
                let net = match game.turn {
                    Color::White => net1,
                    Color::Black => net2,
                    Color::None => break
                };
                let input: Vec<f64> = game.board.iter().map(|c| match c {
                    &Color::None => 0.,
                    &c => if c == game.turn {
                        1.
                    } else {
                        -1.
                    }
                }).collect();
                let mut topo = net.compute(&input).iter().enumerate().collect::<Vec<_>>();
                topo.sort_unstable_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap());
                for i in topo.iter().map(|(x,_)| x) {
                    let pos = (i/MAX_WIDTH, i%MAX_HEIGHT);
                    match game.play(pos) {
                        Ok(c) => winner = c,
                        Err(GameError::IllegalMove) => {
                            continue
                        },
                        _ => panic!()
                    }
                    dbg!(i, pos);
                    break
                }
            }
        }
        scores
    }
}

pub struct AI;
impl AI {
    pub fn new(color: Color) -> Self {
        Self
    }
    pub fn train(&mut self) {

    }
}