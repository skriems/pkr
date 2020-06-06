use crate::card::*;
use crate::error::Result;
use crate::holding::*;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// A Deck of Cards
pub struct Deck {
    pub cards: [Card; 52],
    rng: ThreadRng,
}

impl Deck {
    pub fn new() -> Self {
        let cards: [Card; 52] = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
        ];

        let rng = ThreadRng::default();
        Deck { cards, rng }
    }

    pub fn shuffle(&mut self) {
        let mut rng = self.rng;
        let mut cards = self.cards;
        cards.shuffle(&mut rng);
        self.cards = cards;
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
        assert_eq!(Deck::new().cards.len(), 52);
    }
}
