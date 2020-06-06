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
    idx: u8,
}

impl<'a> Deck<'a> {
    pub fn new(cards: &'a mut [Card]) -> Result<Self> {
        if cards.len() != 52 {
            return Err(Error::InvalidDeck);
        }

        let mut rng = ThreadRng::default();
        let idx = 0;
        Ok(Deck { cards, rng, idx })
    }

    pub fn shuffled(cards: &'a mut [Card]) -> Result<Self> {
        if cards.len() != 52 {
            return Err(Error::InvalidDeck);
        }

        let mut rng = ThreadRng::default();
        let idx = 0;

        cards.shuffle(&mut rng);
        Ok(Deck { cards, rng, idx })
    }

    pub fn get_holding(&mut self) -> Result<Holding> {
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
