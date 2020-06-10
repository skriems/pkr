//! This module holds the `HandResult` struct which takes an optional `Board` and computes it's `HandResult`
use crate::card::*;
use crate::hand::*;
use crate::holding::*;

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
    use crate::deck::*;
    use super::*;

    #[test]
    fn basic() {
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
    fn pair_flop() {
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
    fn pair_turn() {
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
    fn pair_river() {
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
    fn two_pair_flop() {
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
    fn two_pair_turn() {
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
    fn two_pair_river() {
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
    fn trips_paired_flop() {
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
    fn trips_turn() {
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
    fn trips_river() {
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
    fn trips_pocket_pair_flop() {
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
    fn trips_pocket_pair_turn() {
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
    fn trips_pocket_pair_river() {
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
    fn quads_turn() {

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
    fn quads_pocket_pair_flop() {
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
    fn quads_pocket_pair_turn() {
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
    fn quads_pocket_pair_river() {
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
    fn fullhouse_flop() {
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
    fn fullhouse_turn() {
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
    fn fullhouse_river() {
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
    fn flush_flop() {
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
    fn flush_turn() {
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
    fn double_paired_board() {
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
        assert_eq!(std::mem::size_of::<HandRank>(), 1);
        assert_eq!(std::mem::size_of::<HandResult>(), 72);
    }

}

