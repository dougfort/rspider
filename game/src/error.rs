use crate::Move;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum GameError {
    #[fail(display = "invalid deal to empty column")]
    DealToEmptyColumn {},

    #[fail(display = "Invalid Move {:?}", mv)]
    InvalidMove { mv: Move },

    #[fail(display = "no checkpoints to undo")]
    NoCheckpointsToUndo {},

    #[fail(display = "unknown checkpoint")]
    UnknownCheckpoint {},

    #[fail(display = "deal from empty deck")]
    DealFromEmptyDeck {},

    #[fail(display = "rewind into the future")]
    RewindIntoFuture {},
}
