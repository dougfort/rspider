use failure::Fail;
use game::delta::Delta;

#[derive(Debug, Fail)]
pub enum ClientError {
    #[fail(display = "unknown Delta {:?}", delta)]
    UnknownDelta { delta: Delta },

    #[fail(display = "no move found in origin")]
    NoMove {},

    #[fail(display = "bottom card in dest is not visible")]
    BottomNotVisible {},
}
