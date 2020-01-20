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

impl From<Suit> for u8 {
    fn from(suit: Suit) -> u8 {
        match suit {
            Suit::Clubs => b'C',
            Suit::Diamonds => b'D',
            Suit::Hearts => b'H',
            Suit::Spades => b'S',
        }
    }
}

pub fn successor(s: Suit) -> Option<Suit> {
    use self::Suit::*;
    match s {
        Clubs => Some(Diamonds),
        Diamonds => Some(Hearts),
        Hearts => Some(Spades),
        Spades => None,
    }
}

pub fn first() -> Suit {
    Suit::Clubs
}

pub fn iter() -> Iter<Suit> {
    Iter {
        current: Some(first()),
    }
}

pub struct Iter<Suit> {
    current: Option<Suit>,
}

impl Iterator for Iter<Suit> {
    type Item = Suit;

    fn next(&mut self) -> Option<Suit> {
        let prev = self.current;
        if let Some(previous) = prev {
            self.current = successor(previous);
            prev
        } else {
            None
        }
    }
}
