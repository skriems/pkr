//! This module holds the `Deck` which either returns itself shuffled
//! or if it was just baught in the store /shrug

use crate::card::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

fn get_cards() -> [Card; 52] {
    [
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
    ]
}

pub struct Deck {
    pub cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = get_cards();
        let mut rng = ThreadRng::default();
        cards.shuffle(&mut rng);
        Deck { cards }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck { cards: get_cards() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let deck = Deck::default();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn new_shuffled() {
        let deck1 = Deck::new();
        let deck2 = Deck::new();
        assert_ne!(deck1.cards[0], deck2.cards[0]);
    }

    #[test]
    fn default() {
        let deck1 = Deck::default();
        let deck2 = Deck::default();
        assert_eq!(deck1.cards[0], deck2.cards[0]);
    }
}
