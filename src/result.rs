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
    /// similar to the `Board` we store the holding cards `Suits` in an usize array
    suits: [usize; 4],
    /// the `Board`
    board: &'a Board<'a>,
}

impl<'a> HandResult<'a> {
    pub fn new(holding: &'a Holding, board: &'a Board) -> Self {

        let mut suits = [0, 0, 0, 0];
        for card in holding.cards {
            suits[card.suit as usize] += 1;
        }

        // let mut hits = [0, 0];
        // // [0, 0] - HighCard or pocket_pair
        // // [1, 0] - Pair or Trips(if pocket_pair)
        // // [0, 1] - Pair
        // // [1, 1] - TwoPair
        // // [2, 0] - Quads or Trips(!pocket_pair or flop.is_paired)
        // // [0, 2] - Trips
        // let pocket_pair = holding.is_pocket_pair();
        // let mut rank = HandRank::HighCard;

        // if let Some(flop) = flop {
        //     for card in flop.cards {
        //         suits[card.suit as usize] += 1;
        //         if holding.cards.contains(card) {
        //             if pocket_pair {
        //                 hits[0] += 1;
        //             } else if card == holding.high_card(){
        //                 hits[0] += 1;
        //             } else {
        //                 hits[1] += 1;
        //             }
        //         }
        //     }
        // }

        // if let Some(turn) = turn {
        //     suits[turn.suit as usize] += 1;
        //     if holding.cards.contains(turn) {
        //         if turn == holding.high_card(){
        //             hits[0] +=1;
        //         } else {
        //             hits[1] +=1;
        //         }
        //     }
        // }

        // if let Some(river) = river {
        //     suits[river.suit as usize] += 1;
        //     if holding.cards.contains(river) {
        //         if river == holding.high_card() {
        //             hits[0] +=1;
        //         } else {
        //             hits[1] +=1;
        //         }
        //     }
        // }

        // let sum = hits[0] + hits[1];

        // if suits[0] > 4 || suits[1] > 4 || suits[2] > 4 || suits[3] > 4 {
        //     rank = HandRank::Flush;
        // } else if sum > 0 {
        //     if sum == 1 {
        //         if holding.is_pocket_pair() {
        //             rank = HandRank::Trips;
        //         } else {
        //             rank = HandRank::Pair;
        //         }
        //     } else if hits[0] == 1 && hits[1] == 1 {
        //         rank = HandRank::TwoPair;
        //     } else if sum == 2 {
        //         if hits[0] == 2 {
        //             if pocket_pair {
        //                 rank = HandRank::Quads;
        //             } else {
        //                 rank = HandRank::Trips;
        //             }
        //         } else if hits[1] == 2 {
        //             rank = HandRank::Trips;
        //         } else {
        //             rank = HandRank::TwoPair;
        //         }
        //     } else if sum == 3 {
        //         if hits[0] == 1 || hits[1] == 1 {
        //             rank = HandRank::FullHouse;
        //         } else {
        //             rank = HandRank::Quads;
        //         }
        //     }
        // } else if holding.is_pocket_pair() {
        //     rank = HandRank::Pair;
        // }
        HandResult { holding, suits, board }
    }

    pub fn rank(&self) -> HandRank {
        let board = self.board.texture();

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
            return HandRank::TwoPair;
        }

        if board.is_paired {
            // - FullHouse
            for pair in self.board.pairs().iter() {
                if let Some(rank) = pair {
                    let card = Card::new(*rank, Suit::Clubs);
                    if self.holding.cards.contains(&card) {
                        // we could have a FullHouse but which of the holding cards
                        // has hit the pair?
                        if self.holding.high_card() == &card {
                            if self.board.flop().contains(self.holding.low_card()) {
                                // we flopped the FullHouse!
                                return HandRank::FullHouse;
                            }
                            if self.board.flop().contains(self.holding.high_card()) {
                                // we flopped the FullHouse!
                                return HandRank::FullHouse;
                            }
                            if self.holding.cards.contains(self.board.turn()) ||
                                self.holding.cards.contains(self.board.river()) {
                                // FIXME: FullHouse via Turn or River but
                                // they might not have been dealt yet!
                                return HandRank::RoyalFlush; // FIXME
                            }
                        } else if self.holding.low_card() == &card {
                            if self.board.flop().contains(self.holding.high_card()) {
                                // we flopped the FullHouse!
                                return HandRank::FullHouse;
                            }
                            if self.board.flop().contains(self.holding.low_card()) {
                                // we flopped the FullHouse!
                                return HandRank::FullHouse;
                            }
                            if self.holding.cards.contains(self.board.turn()) ||
                                self.holding.cards.contains(self.board.river()) {
                                // FIXME: FullHouse via Turn or River but
                                // they might not have been dealt yet!
                                return HandRank::RoyalFlush; // FIXME
                            }
                        }
                    }
                    if self.holding.cards.contains(self.board.turn()) ||
                        self.holding.cards.contains(self.board.river()) {
                        return HandRank::TwoPair;
                    }
            }
            }
            // - Trips
            // - TwoPair High
        }

        HandRank::HighCard
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn pair_flop() {
    //     let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
    //     let holding = Holding::new(&holding_cards).unwrap();

    //     let board_cards = [
    //         Card::from("7c").unwrap(),
    //         Card::from("2s").unwrap(),
    //         Card::from("Kd").unwrap(),
    //         Card::from("5d").unwrap(),
    //         Card::from("7c").unwrap(),
    //     ];
    //     let board = Board::new(&board_cards).full();
    //     let result = HandResult::new(&holding, &board);
    //     assert_eq!(result.rank(), HandRank::Pair);

    //     let board_cards = [
    //         Card::from("7c").unwrap(),
    //         Card::from("2s").unwrap(),
    //         Card::from("Ad").unwrap(),
    //         Card::from("5d").unwrap(),
    //         Card::from("7c").unwrap(),
    //     ];
    //     let board = Board::new(&board_cards).full();
    //     let result = HandResult::new(&holding, &board);

    //     assert_eq!(result.rank(), HandRank::Pair);
    // }

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
        let result = HandResult::new(&holding, &board);
        assert_eq!(result.rank(), HandRank::Quads);
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
        let result = HandResult::new(&holding, &board);
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        let result = HandResult::new(&holding, &board);
        assert_eq!(result.rank(), HandRank::FullHouse);
    }

    // TODO
    // [8♦ 4❤], [5♦ 3♠] | 5♠ 6♠ A♦ | 5❤ | 7♣	[5♦ 3♠] wins with FullHouse

    // [T♦ 6♦], [A♦ 7♦] | Q♦ 8♣ A♠ | 7❤ | 7♠	[A♦ 7♦] wins with TwoPair
    #[test]
    fn full_house_paired_board_on_river() {
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
        let result = HandResult::new(&holding, &board);
        println!("{:#?}", board.ranks);
        println!("{:#?}", board.texture());
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        let result = HandResult::new(&holding, &board);
        println!("{:#?}", board.ranks);
        println!("{:#?}", board.texture());
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        let result = HandResult::new(&holding, &board);
        println!("{:#?}", board.texture());
        println!("{:#?}", board.ranks);
        println!("{:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::TwoPair);
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
        let result = HandResult::new(&holding, &board);
        println!("{:#?}", board.texture());
        println!("board.ranks: {:#?}", board.ranks);
        println!("board.pairs: {:#?}", board.pairs());
        println!("{:#?}", board.get_rank(3));
        assert_eq!(result.rank(), HandRank::FullHouse);
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<HandRank>(), 1);
        assert_eq!(std::mem::size_of::<HandResult>(), 48);
    }

}

