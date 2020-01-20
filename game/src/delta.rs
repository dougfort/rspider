#[derive(Debug, Clone)]
pub enum Delta {
    HiddenCard { index: usize },
    AppendCard { index: usize, card: cards::Card },
    PopCard { index: usize },
}
