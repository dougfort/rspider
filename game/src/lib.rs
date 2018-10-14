// game definitions

extern crate rand;
extern crate cards;

use rand::{Rng, thread_rng};

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

impl Game {
    pub fn new() -> Game {
        let mut reserve = [cards::deck(), cards::deck()].concat();
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
