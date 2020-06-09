use crate::card::*;
use crate::deck::*;
use crate::holding::*;

/// The Flop takes a slice of Cards and is used to determine some characteristics.
#[derive(Debug, PartialEq)]
pub struct Flop<'a> {
    /// well - the cards
    pub cards: &'a [Card],
    /// array storing the number of suits in the flop
    pub suits: [usize;4],
}

impl<'a> Flop<'a> {
    pub fn new(cards: &'a [Card]) -> Self {

        let mut suits = [0, 0, 0, 0];

        for card in cards {
            // using the discriminator as usize for indexing the array
            // Note that we cannot cast to u8 here, b/c indexing is not implemented for u8
            suits[card.suit as usize] +=1;
        }
        Flop { cards, suits }
    }

    pub fn is_suited(&self) -> bool {
        self.suits[0] > 1 ||
        self.suits[1] > 1 ||
        self.suits[2] > 1 ||
        self.suits[3] > 1
    }
}

/// A Hand is dealt for N players and starts with a shuffled deck of cards.
pub struct Hand<'a> {
    /// well - the `Deck` of cards
    pub deck: &'a Deck,
    /// array of `Some(Holding)` for 9-max players
    players: [Option<Holding<'a>>; 9],
    /// the number of players which are actually playing the `Hand`. This is used as offset for
    /// calculating the index positions of the `Flop`, Turn and River cards.
    seats: usize,
}

impl<'a> Hand<'a> {
    pub fn new(deck: &'a Deck) -> Self {
        Hand {
            deck,
            players: [None, None, None, None, None, None, None, None, None],
            seats: 0,
        }
    }

    pub fn deal(&self, n: usize) -> Self {
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
            seats: n,
        }
    }

    /// returns a reference to a players `Holding` cards. Those are stored as `Option` b/c the
    /// player might not exist.
    pub fn get_player(&self, n: usize) -> &Option<Holding> {
        // TODO
        // if n < 1 {
        //     return Err(Error::ParseError);
        // }
        &self.players[n - 1]
    }

    /// returns the `Flop`
    pub fn flop(&self) -> Flop {
        let idx = self.seats * 2;
        Flop::new(&self.deck.cards[idx..idx + 3])
    }

    /// returns the turn card as `&Card`
    pub fn turn(&self) -> &Card {
        let idx = self.seats * 2 + 4;
        &self.deck.cards[idx..idx + 1][0]
    }

    /// returns the river card as `&Card`
    pub fn river(&self) -> &Card {
        let idx = self.seats * 2 + 6;
        &self.deck.cards[idx..idx + 1][0]
    }
}

#[derive(Debug)]
pub enum HandRank {
    HighCard(Rank),         // HighCard
    Pair(Rank),             // HighCard
    TwoPair(Rank, Rank),    // Higher Lower
    ThreeOfAKind(Rank),     // HighCard
    Straight(Rank),         // HighCard
    Flush(Rank),            // HighCard
    FullHouse(Rank, Rank),  // Higher Lower
    FourOfAKind(Rank),      // HighCard
    StraightFlush(Rank),    // HighCard
    RoyalFlush(Rank),       // HighCard
}

/// The HandResult takes a players `Holding` hand, a `Flop`, turn and river card and determines the
/// Hand rank
#[derive(Debug)]
pub struct HandResult<'a> {
    holding: &'a Holding<'a>,
    flop: Option<&'a Flop<'a>>,
    turn: Option<&'a Card>,
    river: Option<&'a Card>,
    pub hits: [u8; 2]
}

impl<'a> HandResult<'a> {
    pub fn new(holding: &'a Holding, flop: Option<&'a Flop>, turn: Option<&'a Card>, river: Option<&'a Card>) -> Self {
        let mut hits: [u8; 2] = [0, 0];

        if let Some(flop) = flop {
            for card in flop.cards {
                if card.rank == holding.cards[0].rank {
                    hits[0] += 1;
                }
                if card.rank == holding.cards[1].rank {
                    hits[1] += 1;
                }
            }

        }

        if let Some(turn) = turn {
            if turn.rank == holding.cards[0].rank {
                hits[0] += 1;
            }
            if turn.rank == holding.cards[1].rank {
                hits[1] += 1;
            }
        }

        if let Some(river) = river {
            if river.rank == holding.cards[0].rank {
                hits[0] += 1;
            }
            if river.rank == holding.cards[1].rank {
                hits[1] += 1;
            }
        }

        HandResult { holding, flop, turn, river, hits }
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
        assert_eq!(hand.seats, 0);
    }

    #[test]
    fn deal() {
        let deck = Deck::new();
        let hand = Hand::new(&deck).deal(2);

        assert_eq!(hand.seats, 2);
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
        // assert that we return the right slice of cards
        let deck = Deck::new();
        let hand = Hand::new(&deck).deal(2);
        let flop = Flop::new(&hand.deck.cards[4..7]);
        assert_eq!(hand.flop(), flop);

        // assert that we calculate the number of suits correctly.
        // since `Deck::default` returns the ordered array of cards, we have only `Suit::Clubs`
        let deck = Deck::default();
        let hand = Hand::new(&deck).deal(2);
        let flop = Flop::new(&hand.deck.cards[4..7]);
        assert_eq!(flop.suits[Suit::Clubs as usize], 3);
        assert_eq!(flop.is_suited(), true);
    }

    #[test]
    fn turn() {
        let deck = Deck::new();
        let hand = Hand::new(&deck).deal(2);
        assert_eq!(hand.turn(), &hand.deck.cards[8..9][0]);
    }

    #[test]
    fn river() {
        let deck = Deck::default();
        let hand = Hand::new(&deck).deal(2);
        assert_eq!(hand.river(), &hand.deck.cards[10..11][0]);
    }

    #[test]
    fn hand_result() {
        // default unshuffled deck
        let deck = Deck::default();

        // player1 gets AKc
        let hand = Hand::new(&deck).deal(1);
        let holding = hand.get_player(1).as_ref().unwrap();

        let expected_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        assert_eq!(holding, &Holding::new(&expected_cards).unwrap());

        // taking the flop from the default deck
        let flop = hand.flop();

        // lets test two pair, or two hits
        let turn = Card::from("As").unwrap();
        let river = Card::from("Ks").unwrap();

        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.hits, [1,1])
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Hand>(), 160);
        assert_eq!(std::mem::size_of::<Flop>(), 48);
        assert_eq!(std::mem::size_of::<HandRank>(), 3);
        assert_eq!(std::mem::size_of::<HandResult>(), 40);
    }

}
