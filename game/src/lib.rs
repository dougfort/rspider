// game definitions

extern crate cards;

#[derive(Debug)]
pub struct Game {
    pub reserve: Vec<cards::Card>,
}

impl Game {
    pub fn new() -> Game {
        let deck = cards::deck();

        Game { reserve: deck }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
