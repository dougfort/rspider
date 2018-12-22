extern crate hex;
extern crate game;
extern crate cards;

pub mod error;

const WIDTH: usize = 10;
type Column = Vec<Option<cards::Card>>;


#[derive(Debug)]
pub struct Client {
    remote: game::Game,
    pub local: Vec<Column>
}

impl Client {
    pub fn new() -> Result<Client, error::ClientError> {
        client_from_game(game::Game::new())
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

        let game = game::Game::from_seed(seed);
        client_from_game(game)
     }

    pub fn seed(&self) -> String {
        hex::encode(self.remote.seed)
    }

} 

fn client_from_game(game: game::Game) -> Result<Client, error::ClientError> {
    let mut client = Client{
        remote: game,
        local: Vec::new(),
    };

    for _ in 0 .. WIDTH {
        client.local.push(Vec::new());
    }

    for delta in client.remote.initial_deltas() {
        match delta {
            game::delta::Delta::HiddenCard{index: i, count: c} => {
                for _ in 0 .. c {
                    client.local[i].push(None);
                }
            }
        }
    };

    Ok(client)
}
