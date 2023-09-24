use std::fs::File;
use std::io::{Write, Read};

use neat_gru::{neural_network::NeuralNetwork, train::Train};

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
        let mut scores: Vec<f64> = Vec::new();
        dbg!(self.nets.len());
        for nets in self.nets.chunks_exact_mut(2) {
            let mut game = Game::new();
            let mut winner = Color::None;
            loop {
                let net = match game.turn {
                    Color::White => &mut nets[0],
                    Color::Black => &mut nets[1],
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
                let topo = net.compute(&input);
                let mut topo = topo.iter().enumerate().collect::<Vec<_>>();
                topo.sort_unstable_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap());
                let mut illegal_counter = 0;
                for i in topo.iter().map(|(x,_)| x) {
                    let pos = (i/MAX_WIDTH, i%MAX_HEIGHT);
                    match game.play(pos) {
                        Ok(c) => winner = c,
                        Err(GameError::IllegalMove) => {
                            illegal_counter+=1;
                            continue
                        },
                        _ => panic!()
                    }
                    break
                }
                if illegal_counter == topo.len() {
                    break
                }
            }
            match winner {
                Color::White => {
                    scores.push(1.);
                    scores.push(-1.);
                },
                Color::Black => {
                    scores.push(-1.);
                    scores.push(1.);
                },
                Color::None => {
                    scores.push(0.);
                    scores.push(0.);
                }
            }
        }
        if scores.len() < self.nets.len() {
            scores.push(0.)
        }
        scores
    }
    fn reset_players(&mut self, nets: Vec<NeuralNetwork<f64>>) {
        self.nets = nets;
    }
    fn post_training(&mut self, mut history: Vec<neat_gru::train::HistoricTopologyLazy<f64>>) {
        let last = history.pop().unwrap();
        let best = last.into_historic().unwrap().topology;
        let mut output = File::create("best_nn.json").expect("Could not create output file");
        write!(output, "{}", best).unwrap();
    }
}

pub struct AI {
    net: NeuralNetwork<f64>
}
impl AI {
    pub fn new(net: NeuralNetwork<f64>) -> Self {
        Self {
            net
        }
    }
    pub fn open() -> Self {
        let path = "best_nn.json";
        let mut content = String::new();
        File::open(path).unwrap().read_to_string(&mut content).unwrap();
        let net: NeuralNetwork<f64> = NeuralNetwork::from_string(&content);
        Self::new(net)
    }
    pub fn train() {
        let mut sim = Simulation::new();
        let mut runner = Train::new(&mut sim);
        runner
            .inputs(MAX_HEIGHT*MAX_WIDTH)
            .outputs(MAX_HEIGHT*MAX_WIDTH)
            .iterations(1000)
            // .max_layers(8 + 2)
            // .max_per_layers(50)
            // .max_individuals(100)
            // .delta_threshold(2.) // Delta parameter from NEAT paper
            // .formula(0.8, 0.8, 0.3) // c1, c2 and c3 from NEAT paper
            .access_train_object(Box::new(|train| {
                let species_count = train.species_count();
                println!("Species count: {}", species_count);
            })) // Callback called after `reset_players` that gives you access to the train object during training
            .start().unwrap(); // .start_async().await for async version
        println!("train finish");
    }
    pub fn play_move(&mut self, game: &mut Game) {
        let input: Vec<f64> = game.board.iter().map(|c| match c {
            &Color::None => 0.,
            &c => if c == game.turn {
                1.
            } else {
                -1.
            }
        }).collect();
        let topo = self.net.compute(&input);
        dbg!(&topo);
        let mut topo = topo.iter().enumerate().collect::<Vec<_>>();
        topo.sort_unstable_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap());
        for i in topo.iter().map(|(x,_)| x) {
            let pos = (i/MAX_WIDTH, i%MAX_HEIGHT);
            match game.play(pos) {
                Ok(_) => return,
                Err(GameError::IllegalMove) => continue,
                _ => panic!()
            }
        }
    }
}