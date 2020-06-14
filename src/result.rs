//! This module holds the `HandResult` struct which takes an optional `Board` and computes it's `HandResult`
use crate::board::*;
use crate::card::*;
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
    /// holding cards for this `HandResult`
    holding: &'a Holding<'a>,
    /// the `Board`
    board: &'a Board<'a>,
    /// the precalculated `BoardTexture`, so we do not need to process it for each `Holding`
    texture: &'a BoardTexture,
    /// similar to the `Board` we store the holding cards `Suits` in an usize array
    suits: [usize; 4],
    /// HighCard from either `Holding` or the `Board`
    pub high_card: &'a Card,
}

impl<'a> HandResult<'a> {
    pub fn new(holding: &'a Holding, board: &'a Board, texture: &'a BoardTexture) -> Self {

        let mut suits = [0, 0, 0, 0];
        for card in holding.cards {
            suits[card.suit as usize] += 1;
        }

        let mut high_card = holding.high_card();
        let board_high = board.high_card();
        if board_high > high_card {
            high_card = board_high;
        }
        HandResult { holding, board, texture, suits, high_card }
    }

    pub fn rank(&self) -> HandRank {
        let board = self.texture;

        if board.has_quads {
            return HandRank::Quads;
        }

        if board.has_flush {
            return HandRank::Flush;
        }

        if board.has_streight {
            return HandRank::Straight;
        }

        if board.has_trips {
            if let Some(rank) = self.board.get_rank(3) {
                // we can use an arbitrary suit here,
                // because `PartialEq` in `contains`
                // only checks the rank
                let card = Card::new(rank, Suit::Clubs);

                // - Quads
                if self.holding.cards.contains(&card) {
                    return HandRank::Quads;
                }

                // - FullHouse
                if board.is_paired || self.holding.is_pocket_pair() {
                    return HandRank::FullHouse;
                }

                // - Trips High
                return HandRank::Trips;
            }
        }

        if board.has_pairs {
            for pair in self.board.pairs().iter() {
                if let Some(rank) = pair {
                    let card = Card::new(*rank, Suit::Clubs);
                    if self.holding.cards.contains(&card) {
                        return HandRank::FullHouse;
                    }
                }
            }
            // TODO: consider pocket pairs?
            return HandRank::TwoPair;
        }

        // let's check if we have a pocket pair and could improve to trips or
        // even Quads. Usefull information even before we know that the
        // board.is_paired
        let is_pocket_pair = self.holding.is_pocket_pair();
        let mut has_trips = false;
        if is_pocket_pair {
            if self.board.flop_dealt {
                for card in self.board.flop() {
                    if self.holding.cards.contains(card) {
                        // we hit our trips! maybe now Quads?
                        if has_trips {
                            return HandRank::Quads;
                        }
                        has_trips = true;
                    }
                }
            }
            if self.board.turn_dealt && self.holding.cards.contains(self.board.turn()) {
                if has_trips {
                    return HandRank::Quads;
                }
                has_trips = true;
            }
            if self.board.river_dealt && self.holding.cards.contains(self.board.river()) {
                if has_trips {
                    return HandRank::Quads;
                }
                has_trips = true;
            }
            // we cannot return Trips here, b/c we have bigger hands to consider below
        }

        if board.is_paired {
            // - FullHouse
            // - Trips
            // - TwoPair
            for pair in self.board.pairs().iter() {
                if let Some(rank) = pair {
                    // taking an arbitrary card of `Rank` here; check PartialEq on `Rank`
                    let card = Card::new(*rank, Suit::Clubs);
                    if self.holding.cards.contains(&card) {

                        if self.holding.high_card() == &card {
                            // we hit the pair with our high card
                            if self.board.cards().contains(self.holding.low_card()) {
                                return HandRank::FullHouse;
                            } else {
                                // TODO can we have a flush here?
                                return HandRank::Trips;
                            }
                        } else if self.holding.low_card() == &card {
                            // we hit the pair with our low card

                            if self.board.cards().contains(self.holding.high_card()) {
                                return HandRank::FullHouse;
                            } else {
                                // TODO can we have a flush here?
                                return HandRank::Trips;
                            }
                        }
                    }

                    // - we have a pair on the board and haven't hit it
                    // - we do not have Quads either
                    // - we can still have a FullHouse if `has_trips`
                    if has_trips {
                        return HandRank::FullHouse;
                    }

                    // we can still have TwoPair
                    if self.holding.is_pocket_pair() {
                        return HandRank::TwoPair;
                    }

                    for card in self.board.cards() {
                        if self.holding.cards.contains(card) {
                            return HandRank::TwoPair;
                        }
                    }
                }
            }
        }

        if board.flush_with_one || board.flush_with_suited {
            let mut flush_suit: Option<Suit> = None;

            // do we have suited cards on board and if so - how many?
            for (idx, number) in self.board.suits.iter().enumerate() {
                if *number >= 3 {
                    flush_suit = Some(Suit::from(idx));
                }
            }

            // if so, do we have that suit in our holding cards? how many?
            if let Some(suit) = flush_suit {
                let mut amount_holding: usize = 0;
                for card in self.holding.cards {
                    if card.suit == suit {
                        amount_holding += 1;
                    }
                }
                if board.flush_with_one && amount_holding >= 1 {
                    return HandRank::Flush;
                } else if board.flush_with_suited && amount_holding == 2 {
                    return HandRank::Flush;
                }
            }
        }

        // TODO Straight

        if has_trips {
            return HandRank::Trips;
        }

        // Now only TwoPair, Pair and HighCards are left
        let mut hits = 0;
        if self.board.river_dealt {
            for card in &self.board.cards[..5] {
                if self.holding.cards.contains(card) {
                    hits += 1;
                }
            }
        } else if self.board.turn_dealt {
            for card in &self.board.cards[..4] {
                if self.holding.cards.contains(card) {
                    hits += 1;
                }
            }
        } else if self.board.flop_dealt {
            for card in &self.board.cards[..3] {
                if self.holding.cards.contains(card) {
                    hits += 1;
                }
            }
        }

        if hits == 2 {
            return HandRank::TwoPair;
        } else if hits == 1 || self.holding.is_pocket_pair() {
            return HandRank::Pair;
        }

        HandRank::HighCard
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank() {
        assert_eq!(HandRank::HighCard < HandRank::Pair, true);
        assert_eq!(HandRank::Pair < HandRank::TwoPair, true);
        assert_eq!(HandRank::TwoPair < HandRank::Trips, true);
        assert_eq!(HandRank::Trips < HandRank::Straight, true);
        assert_eq!(HandRank::Straight < HandRank::Flush, true);
        assert_eq!(HandRank::Flush < HandRank::FullHouse, true);
        assert_eq!(HandRank::FullHouse < HandRank::Quads, true);
        assert_eq!(HandRank::Quads < HandRank::StraightFlush, true);
        assert_eq!(HandRank::StraightFlush < HandRank::RoyalFlush, true);
    }

    #[test]
    fn pair_flop() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("7c").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("Kd").unwrap(),
            Card::from("5d").unwrap(),
            Card::from("3c").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Pair);
    }

    #[test]
    fn two_pair_q3() {
        // [T♦ 3♠], [Q♣ 3♦] | 8♣ 3❤ 7♣ | 9♣ | Q♦	¯\_(ツ)_/¯ Pair vs. Pair
        let holding_cards = [
            Card::from("Qc").unwrap(),
            Card::from("3d").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("8c").unwrap(),
            Card::from("3h").unwrap(),
            Card::from("7c").unwrap(),
            Card::from("9c").unwrap(),
            Card::from("Qd").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::TwoPair);
    }

    #[test]
    fn quads_on_board() {
        let holding_cards = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("Ac").unwrap(),
            Card::from("Jh").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        assert_eq!(rank, HandRank::Quads);
    }

    #[test]
    fn full_house_on_board() {
        let holding_cards = [Card::from("Qc").unwrap(), Card::from("Jc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Kd").unwrap(),
            Card::from("Ac").unwrap(),
            Card::from("Kh").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        assert_eq!(rank, HandRank::FullHouse);
    }

    #[test]
    fn full_house_flopped() {
        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Kd").unwrap(),
            Card::from("7c").unwrap(),
            Card::from("Jh").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        assert_eq!(rank, HandRank::FullHouse);
    }

    #[test]
    fn full_house_paired_board_on_river() {
        // [T♦ 6♦], [A♦ 7♦] | Q♦ 8♣ A♠ | 7❤ | 7♠	[A♦ 7♦] wins with TwoPair
        let holding_cards = [Card::from("Ad").unwrap(), Card::from("7d").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Qd").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("7h").unwrap(),
            Card::from("7s").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board.ranks);
        println!("{:#?}", board.texture());
        assert_eq!(rank, HandRank::FullHouse);
    }

    // [J❤ 5♠], [9♣ 6❤] | 2❤ 5♦ 2♠ | Q❤ | 5❤ 	[J❤ 5♠] wins with TwoPair
    #[test]
    fn full_house_paired_board_and_river() {
        let holding_cards = [Card::from("Jh").unwrap(), Card::from("5s").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("2h").unwrap(),
            Card::from("5d").unwrap(),
            Card::from("2c").unwrap(),
            Card::from("Qh").unwrap(),
            Card::from("5h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board.ranks);
        println!("{:#?}", board.texture());
        assert_eq!(rank, HandRank::FullHouse);
    }

    // TODO two_pairs_paired_board_and_river
    #[test]
    fn two_pairs_paired_board_and_river() {
        // [A♠ K♣], [A♣ 7♣] | 4♦ 3♠ 3♦ | 6♠ | K❤    [A♠ K♣] wins with TwoPair
        let holding_cards = [Card::from("As").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("4d").unwrap(),
            Card::from("3s").unwrap(),
            Card::from("3d").unwrap(),
            Card::from("6s").unwrap(),
            Card::from("Kh").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board.texture());
        println!("{:#?}", board.ranks);
        println!("{:#?}", board.pairs());
        assert_eq!(rank, HandRank::TwoPair);
    }

    #[test]
    fn full_house_pockets_and_board_paired_on_river() {
        // [K♣ 2♣], [8❤ 8♠] | 4❤ A♠ Q♣ | 8♦ | Q♦	[8❤ 8♠] wins with TwoPair
        let holding_cards = [
            Card::from("8h").unwrap(),
            Card::from("8s").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("4h").unwrap(),
            Card::from("As").unwrap(),
            Card::from("Qc").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("Qd").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        assert_eq!(board.flop(), &board_cards[..3]);
        assert_eq!(board.turn(), &board_cards[3]);
        assert_eq!(board.river(), &board_cards[4]);


        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::FullHouse);
    }

    #[test]
    fn trips_on_turn_high_card() {
        // [9♣ 3♠], [8♠ 2♦] | T♠ 9❤ J❤ | 9♦ | K❤	[9♣ 3♠] wins with FullHouse
        let holding_cards = [
            Card::from("9c").unwrap(),
            Card::from("3s").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ts").unwrap(),
            Card::from("9h").unwrap(),
            Card::from("Jh").unwrap(),
            Card::from("9d").unwrap(),
            Card::from("Kh").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Trips);
    }

    #[test]
    fn trips_on_turn_low_card() {
        // [9♣ 3♠], [8♠ 2♦] | T♠ 3❤ J❤ | 3♦ | K❤	[9♣ 3♠] wins with FullHouse
        let holding_cards = [
            Card::from("9c").unwrap(),
            Card::from("3s").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ts").unwrap(),
            Card::from("3h").unwrap(),
            Card::from("Jh").unwrap(),
            Card::from("3d").unwrap(),
            Card::from("Kh").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Trips);
    }

    #[test]
    fn trips_on_river_a6() {
        // [5♠ 3♠], [A❤ 6❤ ] | 4♠ A♣ 3❤ | T♣ | A♦	[A❤ 6❤] wins with FullHouse
        let holding_cards = [
            Card::from("Ah").unwrap(),
            Card::from("6h").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("4s").unwrap(),
            Card::from("Ac").unwrap(),
            Card::from("3h").unwrap(),
            Card::from("Tc").unwrap(),
            Card::from("Ad").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        assert_eq!(board.flop(), &board_cards[..3]);
        assert_eq!(board.turn(), &board_cards[3]);
        assert_eq!(board.river(), &board_cards[4]);


        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Trips);
    }

    #[test]
    fn trips_on_river_kj() {
        // [K♠ J♦], [7♦ 6♠] | K♦ 8❤ 5♠ | 4♠ | K♣	[K♠ J♦] wins with FullHouse
        let holding_cards = [
            Card::from("Ks").unwrap(),
            Card::from("Jd").unwrap(),
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Kd").unwrap(),
            Card::from("8h").unwrap(),
            Card::from("5s").unwrap(),
            Card::from("4s").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Trips);
    }

    #[test]
    fn trips_flopped() {
        // [6♣ 6♠], [9♦ 7❤ ] | A❤ 6❤ 9❤ | 4❤ | K♠	[6♣ 6♠] wins with Pair
        let holding_cards = [
            Card::from("6c").unwrap(),
            Card::from("6s").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("6h").unwrap(),
            Card::from("9h").unwrap(),
            Card::from("4h").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Trips);
    }

    #[test]
    fn flush_with_one_on_turn() {
        // [6♣ 6♠], [9♦ 7❤ ] | A❤ 6❤ 9❤ | 4❤ | K♠	[6♣ 6♠] wins with Pair
        let holding_cards = [
            Card::from("9d").unwrap(),
            Card::from("7h").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("6h").unwrap(),
            Card::from("9h").unwrap(),
            Card::from("4h").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Flush);
    }

    #[test]
    fn high_card() {
        // [J♦ T♣], [7❤ 4♦] | Q♦ J♠ 2❤ | T❤ | 9♦
        let holding_cards = [
            Card::from("7h").unwrap(),
            Card::from("4d").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Qd").unwrap(),
            Card::from("Js").unwrap(),
            Card::from("2h").unwrap(),
            Card::from("Th").unwrap(),
            Card::from("9d").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::HighCard);

        // [K♣ T♦], [K♦ 8♠] | 8♦ 2♣ 6♦ | 9♠ | 5♣	¯\_(ツ)_/¯ Pair vs. Pair
        let holding_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Td").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("8d").unwrap(),
            Card::from("2c").unwrap(),
            Card::from("6d").unwrap(),
            Card::from("9s").unwrap(),
            Card::from("5c").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::HighCard);

        //[K❤ 8♦], [9❤ 8❤] | J♠ 7♦ 2❤ | K♦ | 5♦	¯\_(ツ)_/¯ Pair vs. Pair
        let holding_cards = [
            Card::from("9h").unwrap(),
            Card::from("8h").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Js").unwrap(),
            Card::from("7d").unwrap(),
            Card::from("2h").unwrap(),
            Card::from("Kd").unwrap(),
            Card::from("5d").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::HighCard);
    }

    #[test]
    fn two_pair_vs_two_pair() {
        // [K♠ Q♣], [9♦ 3❤] | 4♠ Q♦ 7❤ | 4♣ | 9❤	[9♦ 3❤] wins with TwoPair
        let board_cards = [
            Card::from("4s").unwrap(),
            Card::from("Qd").unwrap(),
            Card::from("7h").unwrap(),
            Card::from("4c").unwrap(),
            Card::from("9h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [
            Card::from("Ks").unwrap(),
            Card::from("Qc").unwrap()
        ];
        let holding1 = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding1, &board, &texture);
        let rank1 = result1.rank();
        assert_eq!(rank1, HandRank::TwoPair);

        let holding_cards = [
            Card::from("9d").unwrap(),
            Card::from("3h").unwrap()
        ];
        let holding2 = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding2, &board, &texture);
        let rank2 = result2.rank();
        println!("{:#?}", board);
        println!("{:#?}", texture);
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank2, HandRank::TwoPair);

        // `Rank` is equal
        assert_eq!(rank1 > rank2, false);
        assert_eq!(result1.high_card >=result2.high_card, true);

        // [T♠ 5♣], [A♦ 3♣] | A♣ 6♦ 8♣ | 8♦ | 6♣	¯\_(ツ)_/¯ TwoPair vs. TwoPair
        let board_cards = [
            Card::from("Ac").unwrap(),
            Card::from("6d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("6c").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [
            Card::from("Ts").unwrap(),
            Card::from("5c").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        let rank1 = result1.rank();
        assert_eq!(rank1, HandRank::TwoPair);

        let holding_cards = [
            Card::from("Ad").unwrap(),
            Card::from("3c").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        let rank2 = result2.rank();
        assert_eq!(rank2, HandRank::TwoPair);

        println!("{:?}", result1.high_card);
        println!("{:?}", result2.high_card);
        assert_eq!(rank1 > rank2, false);
        assert_eq!(result1.high_card < result2.high_card, true);
    }

    #[test]
    fn two_pairs_with_pocket_pairs() {
        // [Q♦ 5♠], [6♣ 6♦] | T❤ 8♣ 8♠ | K♣ | 4♠	[6♣ 6♦] wins with Pair
        let holding_cards = [
            Card::from("6c").unwrap(),
            Card::from("6d").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Th").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("8s").unwrap(),
            Card::from("Kc").unwrap(),
            Card::from("4s").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::TwoPair);
    }

    #[test]
    fn trips_runner_runner() {
        // [T♦ 8♣], [9♠ 5♦] | 2♠ 4♦ T♣ | 9❤ | 9♦	[9♠ 5♦] wins with FullHouse
        let holding_cards = [
            Card::from("9s").unwrap(),
            Card::from("5d").unwrap()
        ];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("2s").unwrap(),
            Card::from("4d").unwrap(),
            Card::from("Tc").unwrap(),
            Card::from("9h").unwrap(),
            Card::from("9d").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let rank = HandResult::new(&holding, &board, &texture).rank();
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(rank, HandRank::Trips);
    }

    // TODO
    // [7♣ 3♣], [7♠ 5♣] | T♠ Q❤ 4❤ | 6♠ | Q♦	¯\_(ツ)_/¯ HighCard vs. HighCard 

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<HandRank>(), 1);
        assert_eq!(std::mem::size_of::<HandResult>(), 64);
    }
}

