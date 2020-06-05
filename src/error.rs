use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// occurs when we couldn't determine a struct from an `expr: &str`
    ParseError,
    /// occurs when a Holding is created with two equal `Card`s
    InvalidHolding,
    /// can't occur actually
    InvalidDeck,
}
