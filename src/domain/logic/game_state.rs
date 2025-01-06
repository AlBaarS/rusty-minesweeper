use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::{distributions::Uniform, Rng};
use itertools::Itertools;

struct GameState {
    mines: Vec<Vec<bool>>,
    vicinity: Vec<Vec<u8>>,
    revealed: Vec<Vec<bool>>,
    flagged: Vec<Vec<bool>>,
}

impl GameState {
    fn new(mines: Vec<Vec<bool>>, vicinity: Vec<Vec<u8>>, revealed: Vec<Vec<bool>>, flagged: Vec<Vec<bool>>) -> Self {
        Self {
            mines,
            vicinity,
            revealed,
            flagged,
        }
    }

    // Building up the board
    fn generate_seed() -> u64 {
        let mut rng: ThreadRng = rand::thread_rng();
        rng.gen()
    }

    fn generate_mine_indices(number: u8, seed: u64) -> Vec<u8> {
        let mut rng_seeded: StdRng = StdRng::seed_from_u64(seed);
        let range = Uniform::from(0..number);
        let values: Vec<u8> = range.sample_iter(&mut rng_seeded).take(1000).unique().take(40).sorted().collect();
        return values;
    }


}



fn main() {
    let seed: u64 = generate_seed();
    println!("{:?}", generate_mine_indices(255, seed))
}