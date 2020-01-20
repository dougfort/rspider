// game definitions

use self::delta::Delta;
use failure::Error;

pub mod delta;
pub mod error;
pub mod seed;
pub mod source;

use error::GameError::*;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum ColumnCard {
    Visible { card: cards::Card },
    Hidden { card: cards::Card },
}

#[derive(Debug, Clone)]
pub enum Checkpoint {
    Start {
        count: usize,
    },
    Deal {
        count: usize,
    },
    // flipped_hidden_card means there was a hidden card left in the origin column
    // to make  the move we had to flip the card
    // to undo the move, we must flip it back
    Move {
        action: Move,
        flipped_hidden_card: bool,
    },
}

#[derive(Debug)]
pub struct Game {
    source: source::Source,
    columns: Vec<Vec<ColumnCard>>,
    checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
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
    pub fn new() -> Result<Game, Error> {
        Game::from_source(source::Source::new())
    }

    // create a new game from a specified seed
    pub fn from_seed(seed: [u8; 16]) -> Result<Game, Error> {
        Game::from_source(source::Source::from_seed(seed))
    }

    fn from_source(mut source: source::Source) -> Result<Game, Error> {
        let checkpoint_count = source.cards_dealt();

        let mut columns: Vec<Vec<ColumnCard>> = Vec::new();
        for c in initial_counts().iter() {
            let mut column: Vec<ColumnCard> = Vec::new();
            for _ in 0..*c - 1 {
                let wrapped_card = ColumnCard::Hidden {
                    card: source.deal()?,
                };
                column.push(wrapped_card);
            }
            let wrapped_card = ColumnCard::Visible {
                card: source.deal()?,
            };
            column.push(wrapped_card);
            columns.push(column);
        }

        Ok(Game {
            source,
            columns,
            checkpoints: vec![Checkpoint::Start {
                count: checkpoint_count,
            }],
        })
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
        use self::delta::Delta::*;
        let mut deltas: Vec<delta::Delta> = Vec::new();
        for i in 0..WIDTH {
            for column_card in self.columns[i].iter() {
                match column_card {
                    ColumnCard::Hidden { .. } => deltas.push(HiddenCard { index: i }),
                    ColumnCard::Visible { card: c } => {
                        deltas.push(AppendCard { index: i, card: *c })
                    }
                }
            }
        }
        deltas
    }

    pub fn deal(&mut self) -> Result<Vec<delta::Delta>, Error> {
        let mut deltas: Vec<delta::Delta> = Vec::new();
        let checkpoint_count = self.source.cards_dealt();

        // check all the columns first, then we don't have to revert
        // anything on error
        for i in 0..WIDTH {
            if self.columns[i].is_empty() {
                return Err(DealToEmptyColumn {}.into());
            }
        }

        for i in 0..WIDTH {
            let card = self.source.deal()?;
            self.columns[i].push(ColumnCard::Visible { card });
            deltas.push(Delta::AppendCard { index: i, card });
        }

        self.checkpoints.push(Checkpoint::Deal {
            count: checkpoint_count,
        });

        Ok(deltas)
    }

    pub fn move_cards(&mut self, m: Move) -> Result<Vec<delta::Delta>, Error> {
        if !self.is_move_valid(&m) {
            return Err(InvalidMove { mv: m }.into());
        };

        let mut deltas = Vec::<delta::Delta>::new();

        let orig_len = self.columns[m.orig_col].len();
        let orig_cards: Vec<ColumnCard> = self.columns[m.orig_col]
            .drain(orig_len - m.count..)
            .collect();
        for card in orig_cards {
            self.columns[m.dest_col].push(card);
            if let ColumnCard::Visible { card: c } = card {
                deltas.push(delta::Delta::PopCard { index: m.orig_col });
                deltas.push(delta::Delta::AppendCard {
                    index: m.dest_col,
                    card: c,
                });
            }
        }

        // if the origin column now ends with a hidden card,
        // flip it to visible
        let flipped_hidden_card = if self.columns[m.orig_col].is_empty() {
            false
        } else {
            let last_card_index = self.columns[m.orig_col].len() - 1;
            if let ColumnCard::Hidden { card: c } = self.columns[m.orig_col][last_card_index] {
                self.columns[m.orig_col][last_card_index] = ColumnCard::Visible { card: c };
                deltas.push(delta::Delta::PopCard { index: m.orig_col });
                deltas.push(delta::Delta::AppendCard {
                    index: m.orig_col,
                    card: c,
                });
                true
            } else {
                false
            }
        };

        self.checkpoints.push(Checkpoint::Move {
            action: m,
            flipped_hidden_card,
        });

        Ok(deltas)
    }

    pub fn reverse_move_cards(
        &mut self,
        m: Move,
        flipped_hidden_card: bool,
    ) -> Result<Vec<delta::Delta>, Error> {
        let mut deltas = Vec::<delta::Delta>::new();

        // we are moving from dest to orig: this is undo

        // if we flipped the top card, flip it back
        if flipped_hidden_card {
            let last_card_index = self.columns[m.orig_col].len() - 1;
            if let ColumnCard::Visible { card: c } = self.columns[m.orig_col][last_card_index] {
                self.columns[m.orig_col][last_card_index] = ColumnCard::Hidden { card: c };
                deltas.push(delta::Delta::PopCard { index: m.orig_col });
                deltas.push(delta::Delta::HiddenCard { index: m.orig_col });
            }
        }

        let dest_len = self.columns[m.dest_col].len();
        let dest_cards: Vec<ColumnCard> = self.columns[m.dest_col]
            .drain(dest_len - m.count..)
            .collect();
        for card in dest_cards {
            self.columns[m.orig_col].push(card);
            if let ColumnCard::Visible { card: c } = card {
                deltas.push(delta::Delta::PopCard { index: m.dest_col });
                deltas.push(delta::Delta::AppendCard {
                    index: m.orig_col,
                    card: c,
                });
            }
        }

        Ok(deltas)
    }

    pub fn undo(&mut self) -> Result<Vec<delta::Delta>, Error> {
        if self.checkpoints.len() < 2 {
            return Err(NoCheckpointsToUndo {}.into());
        };

        match self.checkpoints.pop() {
            Some(Checkpoint::Deal { count }) => {
                let mut deltas: Vec<delta::Delta> = Vec::new();
                self.source.rewind(count)?;
                for i in 0..WIDTH {
                    self.columns[1].pop();
                    deltas.push(Delta::PopCard { index: i });
                }
                Ok(deltas)
            }
            Some(Checkpoint::Move {
                action,
                flipped_hidden_card,
            }) => self.reverse_move_cards(action, flipped_hidden_card),
            _unknown => Err(UnknownCheckpoint {}.into()),
        }
    }

    pub fn is_move_valid(&self, m: &Move) -> bool {
        if m.orig_col >= WIDTH
            || m.dest_col >= WIDTH
            || m.orig_col == m.dest_col
            || self.columns[m.orig_col].is_empty()
            || m.count > self.columns[m.orig_col].len()
        {
            false
        } else {
            is_move_valid(
                &self.columns[m.orig_col],
                m.count,
                &self.columns[m.dest_col],
            )
        }
    }
}

fn is_move_valid(orig: &[ColumnCard], count: usize, dest: &[ColumnCard]) -> bool {
    let mut orig_cards = Vec::<cards::Card>::new();
    for c_card in orig.iter().skip(orig.len() - count) {
        match c_card {
            ColumnCard::Hidden { .. } => return false,
            ColumnCard::Visible { card: c } => orig_cards.push(*c),
        };
    }
    if cards::is_descending_run(orig_cards.as_slice()) {
        match dest.last() {
            // if the dest column is empty, any move is valid
            None => true,
            Some(dest_cc) => match dest_cc {
                ColumnCard::Hidden { .. } => false,
                ColumnCard::Visible { card: dest_card } => {
                    match cards::rank::successor(orig_cards[0].rank) {
                        None => false,
                        Some(orig_rank_successor) => dest_card.rank == orig_rank_successor,
                    }
                }
            },
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
        use cards::rank::Rank::*;
        use cards::suit::Suit::*;
        use cards::Card;

        let test_items = vec![
            TestData {
                name: "empty dest".to_string(),
                orig: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Ace,
                    },
                }],
                count: 1,
                dest: vec![],
                expected_result: true,
            },
            TestData {
                name: "single: dest not successor".to_string(),
                orig: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Ace,
                    },
                }],
                count: 1,
                dest: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Ace,
                    },
                }],
                expected_result: false,
            },
            TestData {
                name: "single: dest is successor".to_string(),
                orig: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Ace,
                    },
                }],
                count: 1,
                dest: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Two,
                    },
                }],
                expected_result: true,
            },
            TestData {
                name: "multi: dest not successor".to_string(),
                orig: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Five,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Ace,
                        },
                    },
                ],
                count: 1,
                dest: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Ace,
                    },
                }],
                expected_result: false,
            },
            TestData {
                name: "multi: dest is successor".to_string(),
                orig: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Five,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Ace,
                        },
                    },
                ],
                count: 1,
                dest: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Two,
                    },
                }],
                expected_result: true,
            },
            TestData {
                name: "multi: dest is multi and successor".to_string(),
                orig: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Three,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: Two,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: Ace,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: Jack,
                        },
                    },
                ],
                count: 1,
                dest: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: King,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: Queen,
                        },
                    },
                ],
                expected_result: true,
            },
            TestData {
                name: "multi: dest not successor to prev".to_string(),
                orig: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Diamonds,
                            rank: Seven,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: Five,
                        },
                    },
                ],
                count: 2,
                dest: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Diamonds,
                            rank: Queen,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Hearts,
                            rank: Eight,
                        },
                    },
                ],
                expected_result: false,
            },
            TestData {
                name: "run: dest is successor".to_string(),
                orig: vec![
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Two,
                        },
                    },
                    ColumnCard::Visible {
                        card: Card {
                            suit: Clubs,
                            rank: Ace,
                        },
                    },
                ],
                count: 2,
                dest: vec![ColumnCard::Visible {
                    card: Card {
                        suit: Clubs,
                        rank: Three,
                    },
                }],
                expected_result: true,
            },
        ];
        for test_item in test_items {
            assert_eq!(
                is_move_valid(&test_item.orig, test_item.count, &test_item.dest),
                test_item.expected_result,
                "{}",
                test_item.name
            );
        }
    }
}
