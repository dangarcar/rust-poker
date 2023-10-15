use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EngineError {
    #[error("Cannot take card from deck")]
    BadDeckError,
    #[error("Not enough money to bet")]
    NotEnoughMoney,
    #[error("Cannot get the highest card in the hand")]
    HighestCardNotAvailable,
    #[error("The hand must have more than 5 cards")]
    SmallHandError,
    #[error("The game did not ended well")]
    GameNotCompletedSuccessfully,
    #[error("You cannot raise in this round")]
    NoRaiseAllowedError,
    #[error("The blind must be at least one")]
    SmallBlindError,
    #[error("The initial parameters of this game aren't correct")]
    BadGameError,
    #[error("The communication to my player couldn't be done")]
    RecvMyselfError,
}
