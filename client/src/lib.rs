extern crate hex;
extern crate game;
extern crate cards;

use std::error::Error;

use game::delta::Delta;

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

    pub fn total_cards(&self) -> usize {
        self.remote.total_cards()
    }

    pub fn cards_dealt(&self) -> usize {
        self.remote.cards_dealt()
    }

    pub fn checkpoints(&self) -> Vec<game::Checkpoint> {
        self.remote.checkpoints()
    }

    pub fn deal(&mut self) -> std::result::Result<(), Box<Error>> {
        for delta in self.remote.deal()? {
            use game::delta::Delta::*;
            match delta {
                AppendCard{index: i, card: c} => self.local[i].push(Some(c)),
                _ => {
                    return Err(
                        error::ClientError{
                            message: format!("unknown delta {:?}", delta),
                            line: line!(),
                            column: column!(),
                        }.into()
                    );
                }
            }
        }

        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), Box<Error>> {
        let deltas = self.remote.undo()?;
        self.apply_deltas(deltas)?;        
        Ok(())
    }

    fn apply_deltas(&mut self, deltas: Vec<Delta>) -> Result<(), Box<Error>> {
        for delta in deltas {
            match delta {
                Delta::HiddenCard{index: i} => self.local[i].push(None),
                Delta::AppendCard{index: i, card: c} => self.local[i].push(Some(c)),
                Delta::PopCard{index: i} => { self.local[i].pop(); },
            }
        };

        Ok(())
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

    let deltas = client.remote.initial_deltas();

    client.apply_deltas(deltas)?;

    Ok(client)
}

