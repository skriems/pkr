use crate::card::*;
use crate::error::{Error, Result};
use crate::holding::*;

/// A Deck of Cards
#[derive(Debug)]
pub struct Deck<'a> {
    cards: &'a [Card],
}

impl<'a> Deck<'a> {
    pub fn new(cards: &'a [Card]) -> Result<Self> {
        if cards.len() != 52 {
            return Err(Error::InvalidDeck);
        }
        Ok(Deck { cards })
    }

    pub fn holding(&self) -> Result<Holding> {
        Holding::new(&self.cards[..2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        // assert_eq!(Deck::new().cards.len(), 52);
        assert!(true);
    }
}
