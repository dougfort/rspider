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

impl Rank {
    pub fn successor(&self) -> Option<Rank> {
        use self::Rank::*;
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
