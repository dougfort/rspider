// cards definitions

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
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

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn deck() -> Vec<Card> {
    let mut d = vec![];

    for s in vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        for r in vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ] {
            d.push(Card { suit: s, rank: r });
        }
    }

    d
}

#[cfg(test)]
mod tests {
    #[test]
    fn card_order() {
        for (c1, c2) in [
            (
                super::Card {
                    suit: super::Suit::Clubs,
                    rank: super::Rank::Two,
                },
                super::Card {
                    suit: super::Suit::Diamonds,
                    rank: super::Rank::Two,
                },
            ),
            (
                super::Card {
                    suit: super::Suit::Clubs,
                    rank: super::Rank::Queen,
                },
                super::Card {
                    suit: super::Suit::Clubs,
                    rank: super::Rank::King,
                },
            ),
            (
                super::Card {
                    suit: super::Suit::Diamonds,
                    rank: super::Rank::Ace,
                },
                super::Card {
                    suit: super::Suit::Spades,
                    rank: super::Rank::Ace,
                },
            ),
        ].iter()
        {
            assert!(c1 < c2);
        }
    }
    #[test]
    fn deck() {
        let d = super::deck();
        assert_eq!(d.len(), 52);
        let ace_of_clubs = super::Card {suit: super::Suit::Clubs, rank: super::Rank::Ace};
        let mut prev = ace_of_clubs;
        for c in d {
            if c > ace_of_clubs {
                assert!(c > prev);
                prev = c;
            }
        }
    }
}
