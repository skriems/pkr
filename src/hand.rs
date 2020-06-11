use crate::board::*;
use crate::deck::*;
use crate::holding::*;


/// A Hand takes a with a `Deck` and is dealt for 9-max players.
pub struct Hand<'a> {
    /// well - the `Deck` of cards
    pub deck: &'a Deck,
    /// array of `Some(Holding)` for 9-max players
    holdings: [Option<Holding<'a>>; 9],
    /// the `Board`
    pub board: Board<'a>,
}

impl<'a> Hand<'a> {
    pub fn new(deck: &'a Deck, players: usize) -> Self {

        let mut holdings = [None, None, None, None, None, None, None, None, None];
        let mut offset: usize = 0;

        for i in 0..players {
            holdings[i] = Some(Holding::new(&deck.cards[offset..offset + 2]).unwrap());
            offset += 2;
        }

        Hand {
            deck,
            holdings,
            board: Board::new(&deck.cards[offset..]),
        }
    }

    /// returns a reference to a players `Holding` cards. Those are stored as `Option` b/c the
    /// player might not exist.
    pub fn get_player(&self, n: usize) -> &Option<Holding> {
        // TODO
        // if n < 1 {
        //     return Err(Error::ParseError);
        // }
        &self.holdings[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // using the unshuffled default Deck
        let deck = Deck::default();
        let hand = Hand::new(&deck, 2);

        assert_eq!(hand.deck.cards.len(), 52);

        assert_eq!(
            hand.holdings[0],
            Some(Holding::new(&deck.cards[..2]).unwrap())
        );

        assert_eq!(
            hand.holdings[1],
            Some(Holding::new(&deck.cards[2..4]).unwrap())
        );
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Hand>(), 312);
    }
}
