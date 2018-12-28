extern crate hex;
extern crate game;
extern crate cards;

use std::error::Error;

pub mod error;

const WIDTH: usize = 10;
type Column = Vec<Option<cards::Card>>;

#[derive(Debug)]
pub struct Client {
    remote: game::Game,
    pub local: Vec<Column>
}

impl Client {
    pub fn new() -> Result<Client, Box<Error>> {
        client_from_game(game::Game::new()?)
    }

    pub fn from_hex(hex_seed: &str) -> Result<Client, Box<Error>> {
        let seed = game::seed::from_hex(hex_seed)?;

        let game = game::Game::from_seed(seed)?;
        client_from_game(game)
     }

    pub fn seed(&self) -> String {
        hex::encode(self.remote.seed())
    }

    pub fn cards_dealt(&self) -> usize {
        self.remote.cards_dealt()
    }

} 

fn client_from_game(game: game::Game) -> Result<Client, Box<Error>> {
    let mut client = Client{
        remote: game,
        local: Vec::new(),
    };

    for _ in 0 .. WIDTH {
        client.local.push(Vec::new());
    }

    for delta in client.remote.initial_deltas() {
        use game::delta::Delta::*;
        match delta {
            HiddenCard{index: i} => client.local[i].push(None),
            AppendCard{index: i, card: c} => client.local[i].push(Some(c)),
        }
    };

    Ok(client)
}
