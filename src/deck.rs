use crate::card::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

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
