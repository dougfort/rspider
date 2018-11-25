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

pub fn deck() -> Vec<Card> {
    use suit::Suit::*;
    use rank::Rank;

    let mut d = vec![];

    for s in vec![Clubs, Diamonds, Hearts, Spades] {
        for r in Rank::iter() {
            d.push(Card { suit: s, rank: r });
        }
    }

    d
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
        let d = deck();
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
