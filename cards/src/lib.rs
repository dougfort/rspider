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

#[cfg(test)]
mod tests {
    use super::*;
    use suit::Suit::*;
    use rank::Rank::*;

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
