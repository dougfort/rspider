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

impl Suit {
    pub fn successor(&self) -> Option<Suit> {
        use self::Suit::*;
        let s: Option<Suit>  = match self {
            Clubs => Some(Diamonds),
            Diamonds => Some(Hearts),
            Hearts => Some(Spades),
            Spades => None,
        };
        s
    }
}

pub struct Iter<Suit> {
    current: Option<Suit>
}

impl Iterator for Iter<Suit> {
    type Item = Suit;

    fn next(&mut self) -> Option<Suit> {
        let prev = self.current;
        if let Some(previous) = prev {
            self.current = previous.successor();
            prev
        } else {
            None
        }
    }
}

impl Suit {
    pub fn iter() -> Iter<Suit> {
        Iter{
            current: Some(Suit::Clubs)
        }
    }
}

