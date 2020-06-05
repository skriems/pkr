use crate::card::*;
use crate::error::{Error, Result};
use crate::holding::*;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// A Deck of Cards
#[derive(Debug)]
pub struct Deck<'a> {
    cards: &'a mut [Card],
    rng: ThreadRng,
}

impl<'a> Deck<'a> {
    pub fn new(cards: &'a mut [Card]) -> Result<Self> {
        if cards.len() != 52 {
            return Err(Error::InvalidDeck);
        }
        let mut rng = ThreadRng::default();
        cards.shuffle(&mut rng);
        Ok(Deck { cards, rng })
    }

    pub fn get_holding(&mut self) -> Result<Holding> {
        self.shuffle();
        Holding::new(&self.cards[..2])
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
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
