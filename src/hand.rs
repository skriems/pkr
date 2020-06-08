use crate::card::*;
use crate::deck::*;
use crate::holding::*;

/// A Hand is dealt for N players and starts with a shuffled deck of cards.
pub struct Hand<'a> {
    pub deck: &'a Deck,
    players: [Option<Holding<'a>>; 9],
    n_players: usize,
}

impl<'a> Hand<'a> {
    pub fn new(deck: &'a Deck) -> Self {
        Hand {
            deck,
            players: [None, None, None, None, None, None, None, None, None],
            n_players: 0,
        }
    }

    pub fn with_players(&self, n: usize) -> Self {
        // TODO
        // if n < 1 {
        //     return Err(Error::ParseError);
        // }
        let mut players = [None, None, None, None, None, None, None, None, None];

        let mut offset = 0;
        for i in 0..n {
            players[i] = Some(Holding::new(&self.deck.cards[offset..offset + 2]).unwrap());
            offset += 2;
        }

        Hand {
            deck: self.deck,
            players,
            n_players: n,
        }
    }

    pub fn get_player(&self, n: usize) -> &Option<Holding> {
        // TODO
        // if n < 1 {
        //     return Err(Error::ParseError);
        // }
        &self.players[n - 1]
    }

    pub fn flop(&self) -> &[Card] {
        let idx = self.n_players * 2;
        &self.deck.cards[idx..idx + 3]
    }

    pub fn turn(&self) -> &[Card] {
        let idx = self.n_players * 2 + 4;
        &self.deck.cards[idx..idx + 1]
    }

    pub fn river(&self) -> &[Card] {
        let idx = self.n_players * 2 + 6;
        &self.deck.cards[idx..idx + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // using the unshuffled default Deck
        let deck = Deck::default();
        let hand = Hand::new(&deck);

        assert_eq!(hand.deck.cards.len(), 52);
        assert_eq!(
            hand.players,
            [None, None, None, None, None, None, None, None, None]
        );
        assert_eq!(hand.n_players, 0);
    }

    #[test]
    fn with_players() {
        let deck = Deck::new();
        let hand = Hand::new(&deck).with_players(2);

        assert_eq!(hand.n_players, 2);
        assert_eq!(hand.deck.cards.len(), 52);

        assert_eq!(
            hand.players[0],
            Some(Holding::new(&deck.cards[..2]).unwrap())
        );

        assert_eq!(
            hand.players[1],
            Some(Holding::new(&deck.cards[2..4]).unwrap())
        );
    }

    #[test]
    fn flop() {
        let deck = Deck::new();
        let hand = Hand::new(&deck).with_players(2);
        assert_eq!(hand.flop(), &hand.deck.cards[4..7]);
    }

    #[test]
    fn turn() {
        let deck = Deck::new();
        let hand = Hand::new(&deck).with_players(2);
        assert_eq!(hand.turn(), &hand.deck.cards[8..9]);
    }

    #[test]
    fn river() {
        let deck = Deck::default();
        let hand = Hand::new(&deck).with_players(2);
        assert_eq!(hand.river(), &hand.deck.cards[10..11]);
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Hand>(), 160);
    }
}
