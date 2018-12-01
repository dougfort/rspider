// cards definitions

use std::fmt;

pub mod suit;
pub mod rank;


#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: suit::Suit,
    pub rank: rank::Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.rank, self.suit)
    }
}

impl Card {
    pub fn successor(&self) -> Option<Card> {
        if let Some(r) = rank::Rank::successor(&self.rank) {
             Some(Card{suit: self.suit, rank: r})
        } else {
            if let Some(s) = suit::Suit::successor(&self.suit) {
                Some(Card{suit: s, rank: rank::Rank::iter().next().unwrap()})
            } else {
                None
            }
        }
    }

}

pub struct Iter<Card> {
    current: Option<Card>
}

impl Iterator for Iter<Card> {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        let prev = self.current;
        if let Some(previous) = prev {
            self.current = previous.successor();
            prev
        } else {
            None
        }
    }
}

impl Card {
    pub fn iter() -> Iter<Card> {
        Iter{
            current: Some(Card{
                suit: suit::Suit::iter().next().unwrap(), 
                rank: rank::Rank::iter().next().unwrap()
            })
        }
    }
}

// is_run returns true if the cards form a sequence: 
//    all the same suit
//    rank in ascending order
// an empty slice is not a run
// a singleton is a run
pub fn is_run(cards: &[Card]) -> bool {
    match cards.len() {
        0 => false,
        1 => true,
        _ => {
            let mut i = 0;
            let mut j = 1;
            while j < cards.len() {
                if cards[i].suit != cards[j].suit {
                    return false;
                };
                match cards[i].successor() {
                    Some(s) => {
                        if cards[j] != s {
                            return false;
                        };
                    },
                    None => { return false; }
                };
                i += 1;
                j += 1;
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use suit::Suit::*;
    use rank::Rank::*;

    #[test]
    fn runs() {
        struct RunTestDataType {
            cards: Vec<Card>,
            is_run: bool,
        }

        let run_test_data = vec![
            RunTestDataType{cards: vec![], is_run: false},
            RunTestDataType{cards: vec![Card{suit: Clubs, rank: Ace}], is_run: true},
            RunTestDataType{
                cards: vec![
                    Card{suit: Clubs, rank: Ace},
                    Card{suit: Diamonds, rank: Two},
                ],
                is_run: false,
            },
            RunTestDataType{
                cards: vec![
                    Card{suit: Clubs, rank: Ace},
                    Card{suit: Clubs, rank: Five},
                ],
                is_run: false,
            },
            RunTestDataType{
                cards: vec![
                    Card{suit: Clubs, rank: Ace},
                    Card{suit: Clubs, rank: Two},
                ],
                is_run: true,
            },
            RunTestDataType{
                cards: vec![
                    Card{suit: Clubs, rank: Ace},
                    Card{suit: Clubs, rank: Two},
                    Card{suit: Clubs, rank: Three},
                    Card{suit: Clubs, rank: Four},
                    Card{suit: Clubs, rank: Five},
                    Card{suit: Clubs, rank: Six},
                    Card{suit: Clubs, rank: Seven},
                ],
                is_run: true,
            },
            RunTestDataType{
                cards: vec![
                    Card{suit: Diamonds, rank: Queen},
                    Card{suit: Diamonds, rank: King},
                ],
                is_run: true,
            },
        ];
        for td in run_test_data {
            assert_eq!(is_run(&td.cards), td.is_run)
        }

    }

    #[test]
    fn card_order() {
        for (c1, c2) in [
            (
                super::Card {
                    suit: Clubs,
                    rank: Two,
                },
                super::Card {
                    suit: Diamonds,
                    rank: Two,
                },
            ),
            (
                super::Card {
                    suit: Clubs,
                    rank: Queen,
                },
                super::Card {
                    suit: Clubs,
                    rank: King,
                },
            ),
            (
                super::Card {
                    suit: Diamonds,
                    rank: Ace,
                },
                super::Card {
                    suit: Spades,
                    rank: Ace,
                },
            ),
        ].iter()
        {
            assert!(c1 < c2);
        }
    }

    #[test]
    fn deck_order() {
        use suit::Suit::*;
        use rank::Rank::*;
        let d: Vec<Card> = self::Card::iter().collect();
        assert_eq!(d.len(), 52);
        let ace_of_clubs = Card {
            suit: Clubs,
            rank: Ace,
        };
        let mut prev = ace_of_clubs;
        for c in d {
            if c > ace_of_clubs {
                assert!(c > prev);
                prev = c;
            }
        }
    }
}
