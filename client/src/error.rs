use game::delta::Delta;
use game::error::GameError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("unknown Delta {:?}", delta)]
    UnknownDelta { delta: Delta },

    #[error("no move found in origin")]
    NoMove {},

    #[error("bottom card in dest is not visible")]
    BottomNotVisible {},

    /// Represents GameError
    #[error(transparent)]
    GameError(#[from] GameError),

    /// Represents HexError
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
}
