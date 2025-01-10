use rand::prelude::*;
use rand::{distributions::Uniform, Rng};
use itertools::*;

pub struct GameGeneration;
impl GameGeneration {
    pub fn generate_seed() -> u64 {
        let mut rng: ThreadRng = rand::thread_rng();
        rng.gen()
    }

    pub fn generate_mine_indices(size: u32, seed: u64) -> Vec<u32> {
        let number_of_squares: u32 = size * size;
        let mut rng_seeded: StdRng = StdRng::seed_from_u64(seed);
        let range = Uniform::from(0..number_of_squares);
        let number_to_generate: u32 = number_of_squares * 100;
        let mine_density: u32 = number_of_squares / 5;
        let values: Vec<u32> = range.sample_iter(&mut rng_seeded).take(number_to_generate.try_into().unwrap()).unique().take(mine_density.try_into().unwrap()).sorted().collect();
        return values;
    }
}

fn main() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mine_indices() {
        let test_seed: u64 = 1234567890;
        let reference_vector = vec![4, 12, 15, 16, 18];
        assert_eq!(GameGeneration::generate_mine_indices(5, test_seed), reference_vector);
    }
}
