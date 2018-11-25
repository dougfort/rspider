// cards definitions

use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Suit::Clubs => "C",
            Suit::Diamonds => "D",
            Suit::Hearts => "H",
            Suit::Spades => "S",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ::Rank::*;
        let s = match self {
            Ace => " A",
            Two => " 2",
            Three => " 3",
            Four => " 4",
            Five => " 5",
            Six => " 6",
            Seven => " 7",
            Eight => " 8",
            Nine => " 9",
            Ten => "10",
            Jack => " J",
            Queen => " Q",
            King => " K",
        };
        write!(f, "{}", s)
    }
}

impl Rank {
    pub fn successor(&self) -> Option<Rank> {
        use ::Rank::*;
        let s: Option<Rank>  = match self {
            Ace => Some(Two),
            Two => Some(Three),
            Three => Some(Four),
            Four => Some(Five),
            Five => Some(Six),
            Six => Some(Seven),
            Seven => Some(Eight),
            Eight => Some(Nine),
            Nine => Some(Ten),
            Ten => Some(Jack),
            Jack => Some(Queen),
            Queen => Some(King),
            King => None,
        };
        s
    }
}

pub struct Iter<Rank> {
    current: Option<Rank>
}

impl Iterator for Iter<Rank> {
    type Item = Rank;

    fn next(&mut self) -> Option<Rank> {
        let prev = self.current;
        if let Some(previous) = prev {
            self.current = previous.successor();
            prev
        } else {
            None
        }
    }
}

impl Rank {
    pub fn iter() -> Iter<Rank> {
        Iter{
            current: Some(Rank::Ace)
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.rank, self.suit)
    }
}

pub fn deck() -> Vec<Card> {
    use ::Suit::*;
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

    #[test]
    fn card_order() {
        for (c1, c2) in [
            (
                super::Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two,
                },
                super::Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Two,
                },
            ),
            (
                super::Card {
                    suit: Suit::Clubs,
                    rank: Rank::Queen,
                },
                super::Card {
                    suit: Suit::Clubs,
                    rank: Rank::King,
                },
            ),
            (
                super::Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace,
                },
                super::Card {
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                },
            ),
        ].iter()
        {
            assert!(c1 < c2);
        }
    }

    #[test]
    fn deck_order() {
        let d = deck();
        assert_eq!(d.len(), 52);
        let ace_of_clubs = Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
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
