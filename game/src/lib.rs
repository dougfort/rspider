// game definitions

extern crate rand;
extern crate cards;

use rand::{Rng, thread_rng};

pub mod error;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Column {
    pub cards_in_play: Vec<cards::Card>,
    pub visible_count: usize,
}

#[derive(Debug)]
pub struct Game {
    pub reserve: Vec<cards::Card>,
    pub layout: Vec<Column>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Move {
    pub orig_col: usize,
    pub dest_col: usize,
    pub count: usize,
}

impl Game {
    pub fn new() -> Game {
        let mut reserve: Vec<cards::Card> = cards::Card::iter().chain(cards::Card::iter()).collect();
        thread_rng().shuffle(&mut reserve);
        
        let layout = [6, 5, 5, 6, 5, 5, 6, 5, 5, 6].iter().map(|n| {            
            Column{
                cards_in_play: reserve.drain(..n).collect(),
                visible_count: 1,
            }
        }).collect();        
    
        Game { 
            reserve: reserve,
            layout: layout,
        }
    }

    pub fn is_move_valid(&self, m: &Move) -> bool {
        let orig = &self.layout[m.orig_col];
        let dest = &self.layout[m.dest_col];

        if m.count == 0 || m.count >= orig.visible_count {
            return false;
        }
        true
    }

    pub fn possible_moves(&self) -> Result<Vec<Move>, error::GameError> {
        let mut moves = Vec::new();

        for j in 1..self.layout.len() {
            for i in 0..j {
                let orig = &self.layout[i];
                for n in 0..orig.visible_count {
                    moves.push(Move{orig_col: i, dest_col: j, count: n+1});
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
