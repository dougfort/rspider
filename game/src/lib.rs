// game definitions

extern crate rand;
extern crate cards;

use rand::{Rng, thread_rng};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Column {
    pub cards_in_play: Vec<cards::Card>,
    pub visible_count: u32,
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
        
        let empty_cards: Vec<cards::Card> = vec![];
        let empty_column = Column{cards_in_play: empty_cards, visible_count: 0};
    
        Game { 
            reserve: reserve,
            layout: vec![empty_column],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
