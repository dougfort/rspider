extern crate cards;

use rand::{Rng, SeedableRng, XorShiftRng};

use seed;

#[derive(Debug)]
pub struct Source {
    pub seed: [u8; 16],
    pub cards: Vec<cards::Card>,   // TODO: hide cards
}

impl Source {
    pub fn new() -> Source {
        let seed = seed::from_random();
        Source::from_seed(seed)
    }

    pub fn from_seed(seed: [u8; 16]) -> Source {
        let mut cards: Vec<cards::Card> = cards::Card::iter().chain(cards::Card::iter()).collect();
        let mut rng = XorShiftRng::from_seed(seed);
        rng.shuffle(&mut cards);
        Source{seed: seed, cards: cards}
    }
}