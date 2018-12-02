extern crate cards;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Column {
    pub cards_in_play: Vec<cards::Card>,
    pub visible_count: usize,
}

impl Column {
    pub fn movable_index(&self) -> Option<usize> {
        match (self.visible_count, self.cards_in_play.len()) {
            (_, 0) => None,
            (0, _) => None,
            (1, _) => Some(self.cards_in_play.len()-1),
            (v, l) if v > l => None,
            (v, l) => {
                let mut j = l-1;
                let mut i = l-2;
                while (l - i) < v {
                    if let Some(s) = self.cards_in_play[j].successor() {
                        if s != self.cards_in_play[i] {
                            break;
                        }
                    } else {
                        break;
                    }
                    j -= 1;
                    i -= 1;
                }
                Some(i)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn movable_slice() {
        use cards::Card;
        use cards::suit::Suit::*;
        use cards::rank::Rank::*;

        struct TestDataType {
            name: String,
            col: Column,
            expected_index: Option<usize>,
        }
        let test_data = vec![
            TestDataType{
                name: String::from("empty column"),
                col: Column{
                    cards_in_play: vec![],
                    visible_count: 0,
                },
                expected_index: None,
            },
            TestDataType{
                name: String::from("single card"),
                col: Column{
                    cards_in_play: vec![Card{suit: Clubs, rank: Ace}], // <--
                    visible_count: 1,
                },
                expected_index: Some(0),
            },
            TestDataType{
                name: String::from("single visible card"),
                col: Column{
                    cards_in_play: vec![
                        Card{suit: Clubs, rank: Two},
                        Card{suit: Clubs, rank: Ace}, // <--
                    ],
                    visible_count: 1,
                },
                expected_index: Some(1),
            },
            TestDataType{
                name: String::from("run of two visible cards"),
                col: Column{
                    cards_in_play: vec![
                        Card{suit: Clubs, rank: Two}, // <--
                        Card{suit: Clubs, rank: Ace},
                    ],
                    visible_count: 2,
                },
                expected_index: Some(0),
            },
            TestDataType{
                name: String::from("run of three cards with two visible"),
                col: Column{
                    cards_in_play: vec![
                        Card{suit: Clubs, rank: Three},
                        Card{suit: Clubs, rank: Two}, // <--
                        Card{suit: Clubs, rank: Ace},
                    ],
                    visible_count: 2,
                },
                expected_index: Some(1),
            },
            TestDataType{
                name: String::from("run of three cards with three visible"),
                col: Column{
                    cards_in_play: vec![
                        Card{suit: Clubs, rank: Three}, // <--
                        Card{suit: Clubs, rank: Two},
                        Card{suit: Clubs, rank: Ace},
                    ],
                    visible_count: 3,
                },
                expected_index: Some(0),
            },
            TestDataType{
                name: String::from("run of two cards with three visible"),
                col: Column{
                    cards_in_play: vec![
                        Card{suit: Spades, rank: Seven},
                        Card{suit: Clubs, rank: Two}, // <--
                        Card{suit: Clubs, rank: Ace},
                    ],
                    visible_count: 3,
                },
                expected_index: Some(1),
            },
        ];
        for td in test_data {
            assert!(td.col.movable_index() == td.expected_index, td.name); 
        }
    }
}
