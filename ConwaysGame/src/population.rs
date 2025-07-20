use std::collections::HashSet;
use rand::Rng;
use crate::pattern::{glider, blinker, toad};

pub fn create_random_population(width: i32, height: i32) -> HashSet<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut population = HashSet::new();

    for _ in 0..50 {
        let x = rng.gen_range(0..width - 5);
        let y = rng.gen_range(0..height - 5);

        let pattern = match rng.gen_range(0..3) {
            0 => glider(x, y),
            1 => blinker(x, y),
            2 => toad(x, y),
            _ => glider(x, y),
        };

        for cell in pattern {
            population.insert(cell);
        }
    }

    population
}
