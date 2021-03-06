use rand::{Rng, SeedableRng, XorShiftRng};

use super::error;
use super::seed;

use error::GameError::*;

#[derive(Debug, Default)]
pub struct Source {
    seed: [u8; 16],
    cards: Vec<cards::Card>,
    next_card: usize,
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
        Source {
            seed,
            cards,
            next_card: 0,
        }
    }

    pub fn seed(&self) -> [u8; 16] {
        self.seed
    }

    // deal deals the next card in the deck
    // It can't be an iterator because we change the internals of Source
    // by incrementing next_card
    pub fn deal(&mut self) -> Result<cards::Card, error::GameError> {
        if self.next_card >= self.cards.len() {
            Err(DealFromEmptyDeck {})
        } else {
            let next = self.next_card;
            self.next_card += 1;
            Ok(self.cards[next])
        }
    }

    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }

    // number_of_cards dealt is used for rewind
    pub fn cards_dealt(&self) -> usize {
        self.next_card
    }

    // rewind resets the internal index back to the point where the number of
    // cards dealt was n.
    pub fn rewind(&mut self, n: usize) -> Result<(), error::GameError> {
        if n > self.next_card {
            Err(RewindIntoFuture {})
        } else {
            self.next_card = n;
            Ok(())
        }
    }
}
