pub enum Delta {
    HiddenCard {index: usize, count: usize},
    AppendCard {index: usize, card: cards::Card},
}
