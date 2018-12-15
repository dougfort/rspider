// game definitions

extern crate rand;
extern crate cards;

use rand::{Rng, SeedableRng, XorShiftRng};

pub mod error;
pub mod column;
pub mod seed;

#[derive(Debug)]
pub struct Game {
    pub seed: [u8; 16],
    pub reserve: Vec<cards::Card>,
    pub layout: Vec<column::Column>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Move {
    pub orig_col: usize,
    pub dest_col: usize,
    pub count: usize,
}

impl Game {    

    // create a new game from a randomly generated seed
    pub fn new() -> Game {
        let seed = seed::from_random();
        Game::from_seed(seed)
    }

    // create a new game from a specified seed
    pub fn from_seed(seed: [u8; 16]) -> Game {
        let mut reserve: Vec<cards::Card> = cards::Card::iter().chain(cards::Card::iter()).collect();
        let mut rng = XorShiftRng::from_seed(seed);
        rng.shuffle(&mut reserve);
        
        let layout = [6, 5, 5, 6, 5, 5, 6, 5, 5, 6].iter().map(|n| {            
            column::Column{
                cards_in_play: reserve.drain(..n).collect(),
                visible_count: 1,
            }
        }).collect();        
    
        Game { 
            seed: seed,
            reserve: reserve,
            layout: layout,
        }
    }

    pub fn is_move_valid(&self, m: &Move) -> bool {
        let orig = &self.layout[m.orig_col];
        let dest = &self.layout[m.dest_col];

        if m.count == 0 || m.count > orig.visible_count {
            return false;
        }

        match orig.movable_index() {
            Some(i) => {
                if cards::is_run(&orig.cards_in_play[i..]) {
                    match orig.cards_in_play[i].rank.successor() {
                        Some(s) => s == dest.cards_in_play[dest.cards_in_play.len()-1].rank,
                        None => false
                    }
                } else {
                    false
                }
            },
            None => false,
        } 

    }

    pub fn possible_moves(&self) -> Result<Vec<Move>, error::GameError> {
        let mut moves = Vec::new();

        for j in 0..self.layout.len() {
            for i in 0..self.layout.len() {
                if i != j {
                    let orig = &self.layout[i];
                    for n in 0..orig.visible_count {
                        let m = Move{orig_col: i, dest_col: j, count: n+1};
                        if self.is_move_valid(&m) {
                            moves.push(m);
                        }
                    }
                }
            }
        }

        Ok(moves)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_size() {
        let x_size: Vec<usize> = vec![6, 5, 5, 6, 5, 5, 6, 5, 5, 6];
        let x_layout_size: usize = x_size.iter().sum();
        let deck_size: usize = 2 * 52;
        let g = Game::new();
        for (l, a) in g.layout.iter().zip(x_size.iter()) {
            assert_eq!(l.visible_count, 1);
            assert_eq!(l.cards_in_play.len(), *a);
        }
        assert_eq!(g.reserve.len(), deck_size - x_layout_size);
    }
}
