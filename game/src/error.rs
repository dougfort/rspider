use crate::Move;
use thiserror::Error;

/// GameError is the base class for all errors that occur during the game.
#[derive(Error, Debug)]
pub enum GameError {
    #[error("invalid deal to empty column")]
    DealToEmptyColumn {},

    #[error("Invalid Move {:?}", mv)]
    InvalidMove { mv: Move },

    #[error("no checkpoints to undo")]
    NoCheckpointsToUndo {},

    #[error("unknown checkpoint")]
    UnknownCheckpoint {},

    #[error("deal from empty deck")]
    DealFromEmptyDeck {},

    #[error("rewind into the future")]
    RewindIntoFuture {},
}
