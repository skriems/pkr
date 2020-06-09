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

    pub fn is_paired(&self) -> bool {
        self.cards[0].rank == self.cards[1].rank ||
        self.cards[0].rank == self.cards[2].rank ||
        self.cards[1].rank == self.cards[2].rank
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

#[derive(Debug,PartialEq, PartialOrd)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush,
    RoyalFlush,
}

/// The HandResult takes a players `Holding` hand, a `Flop`, turn and river card and determines the
/// Hand rank
#[derive(Debug)]
pub struct HandResult<'a> {
    holding: &'a Holding<'a>,
    flop: Option<&'a Flop<'a>>,
    turn: Option<&'a Card>,
    river: Option<&'a Card>,
    hits: [u8; 2],
    suits: [usize; 4],
    pub rank: HandRank,
}

impl<'a> HandResult<'a> {
    pub fn new(holding: &'a Holding, flop: Option<&'a Flop>, turn: Option<&'a Card>, river: Option<&'a Card>) -> Self {

        let mut suits = [0, 0, 0, 0];
        for card in holding.cards {
            suits[card.suit as usize] += 1;
        }

        let mut hits = [0, 0];
        // [0, 0] - HighCard or pocket_pair
        // [1, 0] - Pair or Trips(if pocket_pair)
        // [0, 1] - Pair
        // [1, 1] - TwoPair
        // [2, 0] - Quads or Trips(!pocket_pair or flop.is_paired)
        // [0, 2] - Trips
        let pocket_pair = holding.is_pocket_pair();
        let mut rank = HandRank::HighCard;

        if let Some(flop) = flop {
            for card in flop.cards {
                suits[card.suit as usize] += 1;
                if holding.cards.contains(card) {
                    if pocket_pair {
                        hits[0] += 1;
                    } else if card == holding.high_card(){
                        hits[0] += 1;
                    } else {
                        hits[1] += 1;
                    }
                }
            }
        }

        if let Some(turn) = turn {
            suits[turn.suit as usize] += 1;
            if holding.cards.contains(turn) {
                if turn == holding.high_card(){
                    hits[0] +=1;
                } else {
                    hits[1] +=1;
                }
            }
        }

        if let Some(river) = river {
            suits[river.suit as usize] += 1;
            if holding.cards.contains(river) {
                if river == holding.high_card() {
                    hits[0] +=1;
                } else {
                    hits[1] +=1;
                }
            }
        }

        let sum = hits[0] + hits[1];

        if suits[0] > 4 || suits[1] > 4 || suits[2] > 4 || suits[3] > 4 {
            rank = HandRank::Flush;
        } else if sum > 0 {
            if sum == 1 {
                if holding.is_pocket_pair() {
                    rank = HandRank::Trips;
                } else {
                    rank = HandRank::Pair;
                }
            } else if hits[0] == 1 && hits[1] == 1 {
                rank = HandRank::TwoPair;
            } else if sum == 2 {
                if hits[0] == 2 {
                    if pocket_pair {
                        rank = HandRank::Quads;
                    } else {
                        rank = HandRank::Trips;
                    }
                } else if hits[1] == 2 {
                    rank = HandRank::Trips;
                } else {
                    rank = HandRank::TwoPair;
                }
            } else if sum == 3 {
                if hits[0] == 1 || hits[1] == 1 {
                    rank = HandRank::FullHouse;
                } else {
                    rank = HandRank::Quads;
                }
            }
        } else if holding.is_pocket_pair() {
            rank = HandRank::Pair;
        }
        HandResult { holding, flop, turn, river, hits, suits, rank }
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
    fn hand_result_basic() {
        // default unshuffled deck
        let deck = Deck::default();

        let hand = Hand::new(&deck).deal(1);
        let holding = hand.get_player(1).as_ref().unwrap();
        let expected_holding = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        assert_eq!(holding, &Holding::new(&expected_holding).unwrap());

        // taking the flop from the default deck
        let flop = hand.flop();
        let expected_flop = [
            Card::from("Qc").unwrap(),
            Card::from("Jc").unwrap(),
            Card::from("Tc").unwrap()
        ];
        assert_eq!(flop.cards, expected_flop);
        // we skip one card
        assert_eq!(hand.turn(), &Card::from("8c").unwrap());
        // we skip one card
        assert_eq!(hand.river(), &Card::from("6c").unwrap());
    }

    #[test]
    fn hand_result_pair_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("Kd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Pair);

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("Ad").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Pair);

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("Ad").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Pair);
    }

    #[test]
    fn hand_result_pair_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("Qd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Ks").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::Pair);
    }

    #[test]
    fn hand_result_pair_river() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("Qd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("2d").unwrap();
        let river = Card::from("Ks").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::Pair);
    }

    #[test]
    fn hand_result_two_pair_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Kd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::TwoPair);
    }

    #[test]
    fn hand_result_two_pair_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Qd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Kd").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::TwoPair);
    }

    #[test]
    fn hand_result_two_pair_river() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Qd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Td").unwrap();
        let river = Card::from("Kd").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::TwoPair);
    }

    #[test]
    fn hand_result_trips_paired_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Ad").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Trips);
    }

    #[test]
    fn hand_result_trips_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("2d").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Ad").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::Trips);
    }

    #[test]
    fn hand_result_trips_river() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("2d").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("5d").unwrap();
        let river = Card::from("Ad").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::Trips);
    }

    #[test]
    fn hand_result_trips_pocket_pair_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("9d").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Trips);
    }

    #[test]
    fn hand_result_trips_pocket_pair_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("4d").unwrap(),
            Card::from("9s").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Ad").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::Trips);
    }

    #[test]
    fn hand_result_trips_pocket_pair_river() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("Ks").unwrap(),
            Card::from("9d").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("2s").unwrap();
        let river = Card::from("Ah").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::Trips);
    }

    #[test]
    fn hand_result_quads_turn() {

        // [Q♣ J♣] | J♦ 5♣ J♠ | J❤ | A♠     [Q♣ J♣] wins with FullHouse
        let holding_cards = [Card::from("Qc").unwrap(), Card::from("Jc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("Jd").unwrap(),
            Card::from("5c").unwrap(),
            Card::from("Js").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Jh").unwrap();
        let river = Card::from("As").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::Quads);
    }

    #[test]
    fn hand_result_quads_pocket_pair_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("Ah").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Quads);
    }

    #[test]
    fn hand_result_quads_pocket_pair_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("Kh").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("As").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::Quads);
    }

    #[test]
    fn hand_result_quads_pocket_pair_river() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("7c").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("Kh").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("2s").unwrap();
        let river = Card::from("As").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::Quads);
    }

    #[test]
    fn hand_result_fullhouse_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Ks").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("Ah").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("Kh").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::FullHouse);
    }

    #[test]
    fn hand_result_fullhouse_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Ks").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("Ah").unwrap(),
            Card::from("Qd").unwrap(),
            Card::from("Kh").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Ad").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::FullHouse);
    }

    #[test]
    fn hand_result_fullhouse_river() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Ks").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("Ah").unwrap(),
            Card::from("Qd").unwrap(),
            Card::from("Kh").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("2d").unwrap();
        let river = Card::from("Ad").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        assert_eq!(result.rank, HandRank::FullHouse);
    }

    #[test]
    fn hand_result_flush_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("2c").unwrap(),
            Card::from("7c").unwrap(),
            Card::from("4c").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let result = HandResult::new(&holding, Some(&flop), None, None);
        assert_eq!(result.rank, HandRank::Flush);
    }

    #[test]
    fn hand_result_flush_turn() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("2c").unwrap(),
            Card::from("7d").unwrap(),
            Card::from("4c").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("5c").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), None);
        assert_eq!(result.rank, HandRank::Flush);
    }

    #[test]
    fn hand_result_double_paired_board() {
        // bug: [5♣ 5♠], [J♦ 9♠] | 4♦ 4♠ K♦ | T❤ | K❤    [5♣ 5♠] wins with Pair
        let holding_cards = [Card::from("5c").unwrap(), Card::from("5s").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let flop_cards = [
            Card::from("4d").unwrap(),
            Card::from("4s").unwrap(),
            Card::from("Kd").unwrap()
        ];
        let flop = Flop::new(&flop_cards);
        let turn = Card::from("Th").unwrap();
        let river = Card::from("Kh").unwrap();
        let result = HandResult::new(&holding, Some(&flop), Some(&turn), Some(&river));
        println!("{:#?}", result);
        assert_eq!(result.rank, HandRank::TwoPair);
    }

    #[test]
    fn hand_rank() {
        assert_eq!(HandRank::HighCard == HandRank::HighCard, true);
        assert_eq!(HandRank::Pair > HandRank::HighCard, true);
        assert_eq!(HandRank::TwoPair > HandRank::Pair, true);
        assert_eq!(HandRank::Trips > HandRank::TwoPair, true);
        assert_eq!(HandRank::Straight > HandRank::Trips, true);
        assert_eq!(HandRank::Flush > HandRank::Straight, true);
        assert_eq!(HandRank::FullHouse > HandRank::Flush, true);
        assert_eq!(HandRank::Quads > HandRank::FullHouse, true);
        assert_eq!(HandRank::StraightFlush > HandRank::Quads, true);
        assert_eq!(HandRank::RoyalFlush > HandRank::StraightFlush, true);
    }


    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Hand>(), 160);
        assert_eq!(std::mem::size_of::<Flop>(), 48);
        assert_eq!(std::mem::size_of::<HandRank>(), 1);
        assert_eq!(std::mem::size_of::<HandResult>(), 72);
    }

}
