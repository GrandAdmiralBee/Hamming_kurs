pub mod sevenfour;
pub mod fifteeneleven;
pub mod thirtyonetwentysix;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CyclicError{
    #[error("Error while generating 7-4 cyclic message: {0}")]
    SevenFourError(String),
    #[error("Error while generating 15-11 cyclic message: {0}")]
    FifteenElevenError(String),
    #[error("Error while generating 31-26 cyclic message: {0}")]
    ThirtyOneTwentySixError(String),
}