use failure::Error;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use game::delta::Delta;
use game::Move;

pub mod error;
use error::ClientError::*;

const WIDTH: usize = 10;
type Column = Vec<Option<cards::Card>>;

#[derive(Debug)]
pub struct Client {
    remote: game::Game,
    used: HashMap<String, Move>,
    pub local: Vec<Column>,
}

#[derive(Debug, Clone)]
pub struct PotentialMove {
    pub mv: Move,
    pub is_used: bool,
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
                AppendCard { index: i, card: c } => self.local[i].push(Some(c)),
                _ => {
                    return Err(UnknownDelta { delta }.into());
                }
            }
        }

        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), Error> {
        let deltas = self.remote.undo()?;
        self.apply_deltas(deltas)?;
        self.used.remove(&self.digest());
        Ok(())
    }

    pub fn digest(&self) -> String {
        let mut hasher = Sha256::new();
        for column in &self.local {
            hasher.input([b'|']);
            for card in column {
                let v = match card {
                    Some(c) => {
                        let v: [u8; 2] = (*c).into();
                        v
                    }
                    None => [b'_', 2],
                };
                hasher.input(v);
            }
        }

        hex::encode(hasher.result())
    }

    pub fn possible_moves(&self) -> Result<Vec<PotentialMove>, Error> {
        let is_used = |m| match self.used.get(&self.digest()) {
            Some(u) => u == &m,
            None => false,
        };
        let mut moves = Vec::<PotentialMove>::new();
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
                        } else {
                            cards = cards[1..].to_vec();
                            break 'len;
                        }
                    }
                }
            }
            if count == 0 {
                return Err(NoMove {}.into());
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
                    let gmv = game::Move {
                        orig_col: i,
                        count,
                        dest_col: j,
                    };
                    let pmv = PotentialMove {
                        mv: gmv,
                        is_used: is_used(gmv),
                    };
                    moves.push(pmv);
                    continue;
                }
                match dest[dest.len() - 1] {
                    None => {
                        return Err(BottomNotVisible {}.into());
                    }
                    Some(dc) => {
                        if dc.rank == valid_dest_rank {
                            let gmv = game::Move {
                                orig_col: i,
                                count,
                                dest_col: j,
                            };
                            let move_is_used = is_used(gmv);
                            let pmv = PotentialMove {
                                mv: gmv,
                                is_used: move_is_used,
                            };
                            moves.push(pmv);
                        }
                    }
                }
            }
        }
        Ok(moves)
    }

    pub fn move_cards(&mut self, m: game::Move) -> Result<(), Error> {
        let pre_move_digest = self.digest();

        let deltas = self.remote.move_cards(m)?;
        self.apply_deltas(deltas)?;

        self.used.insert(pre_move_digest, m);

        Ok(())
    }

    fn apply_deltas(&mut self, deltas: Vec<Delta>) -> Result<(), Error> {
        for delta in deltas {
            match delta {
                Delta::HiddenCard { index: i } => self.local[i].push(None),
                Delta::AppendCard { index: i, card: c } => self.local[i].push(Some(c)),
                Delta::PopCard { index: i } => {
                    self.local[i].pop();
                }
            }
        }

        Ok(())
    }
}

fn client_from_game(game: game::Game) -> Result<Client, Error> {
    let mut client = Client {
        remote: game,
        used: HashMap::new(),
        local: Vec::new(),
    };

    for _ in 0..WIDTH {
        client.local.push(Vec::new());
    }

    let deltas = client.remote.initial_deltas();

    client.apply_deltas(deltas)?;

    Ok(client)
}
