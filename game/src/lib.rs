// game definitions

extern crate rand;
extern crate cards;

use std::error::Error;

pub mod error;
pub mod column;
pub mod seed;
pub mod source;
pub mod delta;

#[derive(Debug)]
pub struct Game {
    source: source::Source,
    pub columns: Vec<Vec<column::ColumnCard>>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Move {
    pub orig_col: usize,
    pub dest_col: usize,
    pub count: usize,
}

const WIDTH: usize = 10;

fn initial_counts() -> [usize; WIDTH] {
    [6, 5, 5, 6, 5, 5, 6, 5, 5, 6]
}

impl Game {    

    // create a new game from a randomly generated seed
    pub fn new() -> Result<Game, Box<Error>> {
        Game::from_source(source::Source::new())
    }

    // create a new game from a specified seed
    pub fn from_seed(seed: [u8; 16]) -> Result<Game, Box<Error>> {
        Game::from_source(source::Source::from_seed(seed))
    }

    pub fn from_source(mut source: source::Source) -> Result<Game, Box<Error>> {
        let mut columns: Vec<Vec<column::ColumnCard>> = Vec::new();
        {
            for c in initial_counts().iter() {
                let mut column: Vec<column::ColumnCard> = Vec::new();
                for _ in 0..*c-1 {
                    let wrapped_card = column::ColumnCard::Hidden{card: source.deal()?}; 
                    column.push(wrapped_card);
                }
                let wrapped_card = column::ColumnCard::Visible{card: source.deal()?}; 
                column.push(wrapped_card);
                columns.push(column);
            }
        }
    
        Ok(Game{source: source, columns: columns})

    }

    pub fn seed(&self) -> [u8; 16] {
        self.source.seed()
    }

    pub fn cards_dealt(&self) -> usize {
        self.source.cards_dealt()
    }

    pub fn initial_deltas(&self) -> Vec<delta::Delta> {
        use delta::Delta::*;
        let mut deltas: Vec<delta::Delta> = Vec::new();
        for i in 0..WIDTH {
            for column_card in self.columns[i].iter() {
                match column_card {
                    column::ColumnCard::Hidden{card: _} => {
                        deltas.push(HiddenCard{index: i})
                    },
                    column::ColumnCard::Visible{card: c} => {
                        deltas.push(AppendCard{index: i, card: *c})
                    },
                }
            }
        }
        deltas
    }
}
/*
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
*/