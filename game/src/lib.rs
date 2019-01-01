// game definitions

extern crate rand;
extern crate cards;

use std::error::Error;
use delta::Delta;

pub mod error;
pub mod seed;
pub mod source;
pub mod delta;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum ColumnCard {
    Visible{card: cards::Card},
    Hidden{card: cards::Card},
}

#[derive(Debug, Clone)]
pub enum Checkpoint {
    Start{count: usize},
    Deal{count: usize},
}

#[derive(Debug)]
pub struct Game {
    source: source::Source,
    columns: Vec<Vec<ColumnCard>>,
    checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Move {
    pub orig_col: usize,
    pub count: usize,
    pub dest_col: usize,
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

    fn from_source(mut source: source::Source) -> Result<Game, Box<Error>> {
        let checkpoint_count = source.cards_dealt();

        let mut columns: Vec<Vec<ColumnCard>> = Vec::new();
        for c in initial_counts().iter() {
            let mut column: Vec<ColumnCard> = Vec::new();
            for _ in 0..*c-1 {
                let wrapped_card = ColumnCard::Hidden{card: source.deal()?}; 
                column.push(wrapped_card);
            }
            let wrapped_card = ColumnCard::Visible{card: source.deal()?}; 
            column.push(wrapped_card);
            columns.push(column);
        }
    
        Ok(
            Game{
                source: source, 
                columns: columns, 
                checkpoints: vec![Checkpoint::Start{count: checkpoint_count}],
            },
        )

    }

    pub fn seed(&self) -> [u8; 16] {
        self.source.seed()
    }

    pub fn total_cards(&self) -> usize {
        self.source.total_cards()
    }
    pub fn cards_dealt(&self) -> usize {
        self.source.cards_dealt()
    }

    pub fn checkpoints(&self) -> Vec<Checkpoint> {
        self.checkpoints.to_vec()
    }

    pub fn initial_deltas(&self) -> Vec<delta::Delta> {
        use delta::Delta::*;
        let mut deltas: Vec<delta::Delta> = Vec::new();
        for i in 0..WIDTH {
            for column_card in self.columns[i].iter() {
                match column_card {
                    ColumnCard::Hidden{card: _} => {
                        deltas.push(HiddenCard{index: i})
                    },
                    ColumnCard::Visible{card: c} => {
                        deltas.push(AppendCard{index: i, card: *c})
                    },
                }
            }
        }
        deltas
    }

    pub fn deal(&mut self) -> Result<Vec<delta::Delta>, Box<Error>> {
        let mut deltas: Vec<delta::Delta> = Vec::new();
        let checkpoint_count = self.source.cards_dealt();

        // check all the columns first, then we don't have to revert 
        // anything on error
        for i in 0..WIDTH {
            if self.columns[i].is_empty() {
                return Err(
                    error::GameError{
                        message: "invalid deal to empty column".to_string(),
                        line: line!(),
                        column: column!(),
                    }.into()
                );
            }
        };

        for i in 0..WIDTH {
            let card = self.source.deal()?;
            self.columns[1].push(ColumnCard::Visible{card: card});
            deltas.push(Delta::AppendCard{index: i, card: card});
        };

        self.checkpoints.push(Checkpoint::Deal{count: checkpoint_count});

        Ok(deltas)
    }

    pub fn undo(&mut self) -> Result<Vec<delta::Delta>, Box<Error>> {
        use delta::Delta;
        let mut deltas: Vec<delta::Delta> = Vec::new();

        if self.checkpoints.len() < 2 {
            return Err(
                error::GameError{
                    message: "no checkpoints to undo".to_string(),
                    line: line!(),
                    column: column!(),
                }.into()
            );
        };

        match self.checkpoints.pop() {
            Some(Checkpoint::Deal{count}) => {
                self.source.rewind(count)?;
                for i in 0..WIDTH {
                    self.columns[1].pop();
                    deltas.push(Delta::PopCard{index: i});
                };
            },
            _unknown => {
                return Err(
                    error::GameError{
                        message: format!("unknown checkpoint {:?}", _unknown).to_string(),
                        line: line!(),
                        column: column!(),
                    }.into()
                );
            },
        };

        Ok(deltas)
    }

    pub fn is_move_valid(&self, m: &Move) -> bool {
        if m.orig_col >= WIDTH || 
            m.dest_col >= WIDTH || 
            m.orig_col == m.dest_col ||
            self.columns[m.orig_col].is_empty() ||
            m.count > self.columns[m.orig_col].len() {
            false 
        } else {
            is_move_valid(&self.columns[m.orig_col], m.count, &self.columns[m.dest_col])
        }
    }
}

fn is_move_valid(orig: &[ColumnCard], count: usize, dest: &[ColumnCard]) -> bool {
    let mut orig_cards = Vec::<cards::Card>::new();
    for n in orig.len()-count..orig.len() {
        let c_card = &orig[n];
        match c_card {
            ColumnCard::Hidden{card: _} => return false,
            ColumnCard::Visible{card: c} => orig_cards.push(*c),
        };
    }
    if cards::is_descending_run(orig_cards.as_slice()) {
        match dest.last() {
            // if the dest column is empty, any move is valid
            None => true,
            Some(dest_cc) => {
                match dest_cc {
                    ColumnCard::Hidden{card: _} => false,
                    ColumnCard::Visible{card: dest_card} => {
                        match cards::rank::successor(orig_cards[0].rank) {
                            None => false,
                            Some(orig_rank_successor) => dest_card.rank == orig_rank_successor
                        }
                    }
                }
            }
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestData {
        name: String,
        orig: Vec<ColumnCard>,
        count: usize,
        dest: Vec<ColumnCard>,
        expected_result: bool,
    }    

    #[test]
    fn test_is_move_valid() {
        use cards::Card;
        use cards::suit::Suit::*;
        use cards::rank::Rank::*;

        let test_items = vec![
            TestData{
                name: "empty dest".to_string(),
                orig: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}}],
                count: 1,
                dest: vec![],
                expected_result: true,
            },
            TestData{
                name: "single: dest not successor".to_string(),
                orig: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}}],
                count: 1,
                dest: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}}],
                expected_result: false,
            },
            TestData{
                name: "single: dest is successor".to_string(),
                orig: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}}],
                count: 1,
                dest: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Two}}],
                expected_result: true,
            },
            TestData{
                name: "multi: dest not successor".to_string(),
                orig: vec![
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Five}},
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}},
                ],
                count: 1,
                dest: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}}],
                expected_result: false,
            },
            TestData{
                name: "multi: dest is successor".to_string(),
                orig: vec![
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Five}},
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}},
                ],
                count: 1,
                dest: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Two}}],
                expected_result: true,
            },
            TestData{
                name: "run: dest not successor".to_string(),
                orig: vec![
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Two}},
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}},
                ],
                count: 2,
                dest: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}}],
                expected_result: false,
            },
            TestData{
                name: "run: dest is successor".to_string(),
                orig: vec![
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Two}},
                    ColumnCard::Visible{card: Card{suit: Clubs, rank: Ace}},
                ],
                count: 2,
                dest: vec![ColumnCard::Visible{card: Card{suit: Clubs, rank: Three}}],
                expected_result: true,
            },
        ];
        for test_item in test_items {
            assert_eq!(
                is_move_valid(&test_item.orig, test_item.count, &test_item.dest), 
                test_item.expected_result,
                "{}", test_item.name
            );
        }
    }
}
