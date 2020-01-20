use std::fmt;

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
        use self::Rank::*;
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

impl From<Rank> for u8 {
    fn from(rank: Rank) -> u8 {
        use self::Rank::*;
        match rank {
            Ace => b'A',
            Two => b'2',
            Three => b'3',
            Four => b'4',
            Five => b'5',
            Six => b'6',
            Seven => b'7',
            Eight => b'8',
            Nine => b'9',
            Ten => b'X',
            Jack => b'J',
            Queen => b'Q',
            King => b'K',
        }
    }
}

pub fn successor(r: Rank) -> Option<Rank> {
    use self::Rank::*;
    match r {
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
    }
}

pub fn first() -> Rank {
    Rank::Ace
}

pub struct Iter<Rank> {
    current: Option<Rank>,
}

pub fn iter() -> Iter<Rank> {
    Iter {
        current: Some(first()),
    }
}

impl Iterator for Iter<Rank> {
    type Item = Rank;

    fn next(&mut self) -> Option<Rank> {
        let prev = self.current;
        if let Some(previous) = prev {
            self.current = successor(previous);
            prev
        } else {
            None
        }
    }
}
