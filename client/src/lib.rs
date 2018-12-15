extern crate hex;
extern crate game;

pub mod error;

#[derive(Debug)]
pub struct Client {
    pub game: game::Game
}

impl Client {
    pub fn new() -> Client {
        Client{
            game: game::Game::new(),
        }
    }

    pub fn from_hex(hex_seed: &str) -> Result<Client, error::ClientError> {
        let seed = match game::seed::from_hex(hex_seed) {
            Err(err) => {
                return Err(
                    error::ClientError{
                        message: format!("invalid seed string: {}", err).to_string(),
                        line: line!() as usize,
                        column: column!() as usize
                    }
                )
            },
            Ok(s) => s
        };

        Ok(
            Client{
                game: game::Game::from_seed(seed)
            }
        )
    }

    pub fn seed(&self) -> String {
        hex::encode(self.game.seed)
    }
} 