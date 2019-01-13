extern crate hex;
extern crate failure;
extern crate game;
extern crate cards;

use failure::Error;

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
    pub fn new() -> Result<Client, Error> {
        client_from_game(game::Game::new()?)
    }

    pub fn from_hex(hex_seed: &str) -> Result<Client, Error> {
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

    pub fn deal(&mut self) -> std::result::Result<(), Error> {
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

    pub fn undo(&mut self) -> Result<(), Error> {
        let deltas = self.remote.undo()?;
        self.apply_deltas(deltas)?;        
        Ok(())
    }

    pub fn possible_moves(&self) -> Result<Vec<game::Move>, Error> {
        let mut moves = Vec::<game::Move>::new();
        'width: for i in 0..WIDTH {
            if self.local[i].is_empty() {
                continue 'width;
            }
            let orig = &self.local[i];
            let mut count = 0;
            let mut cards = Vec::<cards::Card>::new();
            'len: for n in (0..orig.len()).rev() {
                match orig[n] {
                    None => break,
                    Some(c) => {
                        cards.insert(0, c);
                        if cards::is_descending_run(&cards) {
                            count += 1;
                        }else {
                            cards = cards[1..].to_vec();
                            break 'len;
                        }
                    } 
                }
            };
            if count == 0 {
                return Err(
                    error::ClientError{
                        message: "no move found in origin".to_string(),
                        line: line!(),
                        column: column!(),
                    }.into()
                );
            }
            let valid_dest_rank = match cards::rank::successor(cards[0].rank) {
                None => continue,
                Some(r) => r,
            };
            for j in 0..WIDTH {
                if j == i {
                    continue;
                }
                let dest = &self.local[j];
                if dest.is_empty() {
                    moves.push(game::Move{orig_col: i, count: count, dest_col: j});                    
                    continue;                      
                }
                match dest[dest.len()-1] {
                    None => {
                        return Err(
                            error::ClientError{
                                message: "bottom card in dest is not visible".to_string(),
                                line: line!(),
                                column: column!(),
                            }.into()
                        );
                    },
                    Some(dc) => {
                        if dc.rank == valid_dest_rank {
                            moves.push(game::Move{orig_col: i, count: count, dest_col: j});                    
                        }
                    }
                }
            }
        }
        Ok(moves)
    }

    pub fn move_cards(&mut self, m: game::Move) -> Result<(), Error> {
        let deltas = self.remote.move_cards(m)?;
        self.apply_deltas(deltas)
    }

    fn apply_deltas(&mut self, deltas: Vec<Delta>) -> Result<(), Error> {
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

fn client_from_game(game: game::Game) -> Result<Client, Error> {
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

