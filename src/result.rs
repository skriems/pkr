//! This module holds the `HandResult` struct which takes an optional `Board` and computes it's `HandResult`
//!
//! TODO:
//! The HandResult should expose an `eval` which processes the `HandRank` onto an Option<HandRank>
//! field
use crate::board::*;
use crate::card::*;
use crate::holding::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandRank {
    HighCard,
    Pair(Rank),
    TwoPair(Rank, Rank),
    Trips(Rank),
    Straight(Rank),
    Flush(usize),
    FullHouse(Rank, Rank),
    Quads(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

/// Return the sum of 5 `Ranks` for a given `Suit`
pub fn suit_rank(ranks: &[[usize; 4]; 13], suit: usize) -> usize {
    let mut rank_sum = 0;
    for (rank, suits) in ranks.iter().rev().enumerate() {
        // if rank > 12 {
        //     rank = 12;
        // }
        if suits[suit] > 0 {
            rank_sum += 12 - rank
        }

        if rank == 4 {
            break;
        }
    }
    rank_sum
}

fn rank(ranks: &[[usize; 4]; 13], num_ranks: &[usize; 13], num_suits: &[usize; 4]) -> HandRank {
    let mut straight_high = 0;
    let mut straight_ace = false;
    let mut last_idx = 0;
    let mut connected = 0;
    let mut has_straight = false;

    let mut observed_suits: [usize; 4] = [0, 0, 0, 0];

    let mut pairs: [Option<Rank>; 2] = [None, None];
    let mut trips: Option<Rank> = None;
    let mut quads: Option<Rank> = None;
    let mut full_house: bool = false;

    // iterating over the num_ranks array
    for (idx, amount) in num_ranks.iter().rev().enumerate() {
        let num = *amount;
        if num == 0 {
            continue;
        }

        if num == 2 {
            if trips.is_some() {
                full_house = true;
            }

            if !pairs[1].is_some() {
                if let Some(_rank) = pairs[0] {
                    pairs[1] = Some(Rank::from(12 - idx));
                } else {
                    pairs[0] = Some(Rank::from(12 - idx));
                }
            }
        }

        if num == 3 {
            trips = Some(Rank::from(12 - idx));
            if pairs[0].is_some() || pairs[1].is_some() {
                full_house = true;
            }
        }

        if num == 4 {
            quads = Some(Rank::from(12 - idx));
        }

        // whereever we start the first observation of a particular card
        // connected is set to Zero and straight_high to the idx position
        // TODO: since `!has_straight` prevents `connectted` from getting reset
        // we could use that to detect A2345 straights
        if num > 0 && !has_straight {
            // reset `connected` and `straight_high` when idx is positive
            // and the last_idx is not the previous one
            if idx > 0 && last_idx != idx - 1 {
                connected = 0;
                straight_high = idx;
            } else if idx == 0 {
                straight_ace = true;
                straight_high = idx;
            } else if idx == 1 && !straight_ace {
                straight_high = idx;
            }
            // increment
            connected += 1;
            last_idx = idx;
        }

        if connected == 5 || connected == 4 && straight_high == 9 && straight_ace {
            has_straight = true;
        }
    }

    // check RoyalFlush, StraightFlush
    if has_straight {
        let mut norm_rank = 12 - straight_high;
        if norm_rank < 5 {
            norm_rank = 5;
        }
        let start = norm_rank - 5;
        // norm_rank + 1 b/c upper bound is exclusive
        for rank in start..norm_rank + 1 {
            for (suit, num) in ranks[rank].iter().enumerate() {
                observed_suits[suit] += num;
            }
        }
        for num_suit in observed_suits.iter() {
            if *num_suit == 5 {
                if straight_high == 0 {
                    return HandRank::RoyalFlush;
                }
                return HandRank::StraightFlush(Rank::from(12 - straight_high));
            }
        }
    }

    // by here we do not have a RoyalFlush or StraightFlush
    // do we have quads?
    if let Some(rank) = quads {
        return HandRank::Quads(rank);
    }

    if full_house {
        if let Some(trips_rank) = trips {
            if let Some(pair1_rank) = pairs[0] {
                if let Some(pair2_rank) = pairs[1] {
                    if pair1_rank > pair2_rank {
                        return HandRank::FullHouse(trips_rank, pair1_rank);
                    } else {
                        return HandRank::FullHouse(trips_rank, pair2_rank);
                    }
                }
                return HandRank::FullHouse(trips_rank, pair1_rank);
            }
        }
    }

    for (suit, num) in num_suits.iter().enumerate() {
        if *num == 5 {
            return HandRank::Flush(suit_rank(ranks, suit));
        }
    }

    if has_straight {
        return HandRank::Straight(Rank::from(12 - straight_high));
    }

    if let Some(rank) = trips {
        return HandRank::Trips(rank);
    }

    // do we have TwoPair or Pair?
    if let Some(pair1_rank) = pairs[0] {
        if let Some(pair2_rank) = pairs[1] {
            if pair1_rank > pair2_rank {
                return HandRank::TwoPair(pair1_rank, pair2_rank);
            }
            return HandRank::TwoPair(pair2_rank, pair1_rank);
        }
        return HandRank::Pair(pair1_rank);
    }
    HandRank::HighCard
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
    /// Array of 13 usize for each respective `Rank`
    pub ranks: [[usize; 4]; 13],
    pub num_ranks: [usize; 13],
    pub num_suits: [usize; 4],
    pub hand_rank: HandRank,
}

impl<'a> HandResult<'a> {
    pub fn new(holding: &'a Holding, board: &'a Board, texture: &'a BoardTexture) -> Self {
        let mut ranks = board.ranks;
        let mut num_ranks = board.num_ranks;
        let mut num_suits = board.num_suits;

        for card in holding.cards {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        let hand_rank = rank(&ranks, &num_ranks, &num_suits);

        HandResult {
            holding,
            board,
            texture,
            ranks,
            num_ranks,
            num_suits,
            hand_rank,
        }
    }

    /// return the sum of `Ranks` for a given `amount` of HighCards
    pub fn high_cards(&self, amount: usize) -> usize {
        let mut rank_sum = 0;
        let mut i = 0;
        for (idx, num) in self.num_ranks.iter().rev().enumerate() {
            if *num == 1 {
                rank_sum += 12 - idx;
                i += num;
            }
            if i == amount {
                break;
            }
        }
        rank_sum
    }
}

impl<'a> PartialEq for HandResult<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.high_cards(5) == other.high_cards(5)
    }
}

impl<'a> PartialOrd for HandResult<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_rank != other.hand_rank {
            return self.hand_rank.partial_cmp(&other.hand_rank);
        }
        match self.hand_rank {
            HandRank::HighCard => {
                return self.high_cards(5).partial_cmp(&other.high_cards(5));
            }
            HandRank::Pair(_) => {
                return self.high_cards(3).partial_cmp(&other.high_cards(3));
            }
            HandRank::TwoPair(_, _) => {
                return self.high_cards(1).partial_cmp(&other.high_cards(1));
            }
            HandRank::Trips(_) => {
                return self.high_cards(2).partial_cmp(&other.high_cards(2));
            }
            HandRank::Quads(_) => {
                return self.high_cards(1).partial_cmp(&other.high_cards(1));
            }
            _ => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_ordering() {
        assert_eq!(
            HandRank::Pair(Rank::Two) < HandRank::Pair(Rank::Three),
            true
        );
        assert_eq!(
            HandRank::TwoPair(Rank::Eight, Rank::Five) < HandRank::TwoPair(Rank::Ace, Rank::Four),
            true
        );
        assert_eq!(
            HandRank::Trips(Rank::Three) < HandRank::Trips(Rank::Four),
            true
        );
        assert_eq!(
            HandRank::Straight(Rank::Jack) < HandRank::Straight(Rank::Queen),
            true
        );
        assert_eq!(
            HandRank::Flush(Rank::King as usize) < HandRank::Flush(Rank::Ace as usize),
            true
        );
        assert_eq!(
            HandRank::FullHouse(Rank::Two, Rank::Five)
                < HandRank::FullHouse(Rank::Three, Rank::Four),
            true
        );
        assert_eq!(
            HandRank::Quads(Rank::Two) < HandRank::Quads(Rank::Three),
            true
        );
        assert_eq!(
            HandRank::StraightFlush(Rank::King) < HandRank::StraightFlush(Rank::Ace),
            true
        );
    }

    #[test]
    fn rank_equlity() {
        assert_eq!(HandRank::Pair(Rank::Two) == HandRank::Pair(Rank::Two), true);
        assert_eq!(
            HandRank::TwoPair(Rank::Three, Rank::Two) == HandRank::TwoPair(Rank::Three, Rank::Two),
            true
        );
        assert_eq!(
            HandRank::Trips(Rank::Three) == HandRank::Trips(Rank::Three),
            true
        );
        assert_eq!(
            HandRank::Straight(Rank::Jack) == HandRank::Straight(Rank::Jack),
            true
        );
        assert_eq!(
            HandRank::Flush(Rank::Ace as usize) == HandRank::Flush(Rank::Ace as usize),
            true
        );
        assert_eq!(
            HandRank::FullHouse(Rank::Three, Rank::Two)
                == HandRank::FullHouse(Rank::Three, Rank::Two),
            true
        );
        assert_eq!(
            HandRank::Quads(Rank::Two) == HandRank::Quads(Rank::Two),
            true
        );
        assert_eq!(
            HandRank::StraightFlush(Rank::Ace) == HandRank::StraightFlush(Rank::Ace),
            true
        );
    }

    #[test]
    fn internal_ranks() {
        // [6♠ 4❤], [K♦ 7❤] | J♦ A♠ 8♦ | 8♣ | 2❤	¯\_(ツ)_/¯ HighCard vs. HighCard
        let holding_cards = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Jd").unwrap(),
            Card::from("As").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        assert_eq!(board.num_ranks, [1, 0, 0, 0, 0, 0, 2, 0, 0, 1, 0, 0, 1]);
        let texture = board.texture();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Pair(Rank::Eight));
        assert_eq!(result.num_ranks, [1, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 1, 1]);
        assert_eq!(board.num_ranks, [1, 0, 0, 0, 0, 0, 2, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    fn high_card() {
        // [J♦ T♣], [7❤ 4♦] | Q♦ J♠ 2❤ | T❤ | 9♦
        let holding_cards = [Card::from("7h").unwrap(), Card::from("4d").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(texture.straight, None);
        assert_eq!(result.hand_rank, HandRank::HighCard);

        // [K♣ T♦], [K♦ 8♠] | 8♦ 2♣ 6♦ | 9♠ | 5♣	¯\_(ツ)_/¯ Pair vs. Pair
        let holding_cards = [Card::from("Kc").unwrap(), Card::from("Td").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::HighCard);

        //[K❤ 8♦], [9❤ 8❤] | J♠ 7♦ 2❤ | K♦ | 5♦	¯\_(ツ)_/¯ Pair vs. Pair
        let holding_cards = [Card::from("9h").unwrap(), Card::from("8h").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);

        assert_eq!(result.hand_rank, HandRank::HighCard);
    }

    #[test]
    fn high_card_vs_high_card() {
        // [A♦ 3♣], [A❤ 8♣] | T♠ 5♠ 9♠ | K♦ | Q♦	¯\_(ツ)_/¯ HighCard vs. HighCard
        let board_cards = [
            Card::from("Ts").unwrap(),
            Card::from("5s").unwrap(),
            Card::from("9s").unwrap(),
            Card::from("Kd").unwrap(),
            Card::from("Qd").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Ad").unwrap(), Card::from("3c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);

        let holding_cards = [Card::from("Ah").unwrap(), Card::from("8c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);

        assert_eq!(result1.hand_rank, HandRank::HighCard);
        assert_eq!(result2.hand_rank, HandRank::HighCard);
        // both have AKQT9 -> split!
        assert_eq!(result1 == result2, true);
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Pair(Rank::King));
    }

    #[test]
    fn pair_vs_pair() {
        // [6♠ 4❤], [K♦ 7❤] | J♦ A♠ 8♦ | 8♣ | 2❤	¯\_(ツ)_/¯ HighCard vs. HighCard
        let holding_cards = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let board_cards = [
            Card::from("Jd").unwrap(),
            Card::from("As").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Pair(Rank::Eight));

        let holding_cards = [Card::from("6s").unwrap(), Card::from("4h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);

        assert_eq!(result2.hand_rank, HandRank::Pair(Rank::Eight));
        assert_eq!(result1 > result2, true);
    }

    #[test]
    fn two_pair() {
        // [T♦ 3♠], [Q♣ 3♦] | 8♣ 3❤ 7♣ | 9♣ | Q♦	¯\_(ツ)_/¯ Pair vs. Pair
        let board_cards = [
            Card::from("8c").unwrap(),
            Card::from("3h").unwrap(),
            Card::from("7c").unwrap(),
            Card::from("9c").unwrap(),
            Card::from("Qd").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Qc").unwrap(), Card::from("3d").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::TwoPair(Rank::Queen, Rank::Three));
    }

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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::TwoPair(Rank::King, Rank::Three));
    }

    #[test]
    fn two_pair_vs_two_pair() {
        // [7♣ 4♦], [9♠ 3♦] | 6❤ 4♣ 6♠ | J♣ | 3♣	¯\_(ツ)_/¯ TwoPair vs. TwoPair
        let board_cards = [
            Card::from("6h").unwrap(),
            Card::from("4c").unwrap(),
            Card::from("6s").unwrap(),
            Card::from("Jc").unwrap(),
            Card::from("3c").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let holding_cards = [Card::from("7c").unwrap(), Card::from("4d").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::TwoPair(Rank::Six, Rank::Four));
        assert_eq!(result1.num_ranks, [0, 1, 2, 0, 2, 1, 0, 0, 0, 1, 0, 0, 0]);

        let holding_cards = [Card::from("9s").unwrap(), Card::from("3d").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.hand_rank, HandRank::TwoPair(Rank::Six, Rank::Three));

        // 64 vs 63
        assert_eq!(result1 > result2, true);

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

        let holding_cards = [Card::from("Ks").unwrap(), Card::from("Qc").unwrap()];
        let holding1 = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding1, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::TwoPair(Rank::Queen, Rank::Four));

        let holding_cards = [Card::from("9d").unwrap(), Card::from("3h").unwrap()];
        let holding2 = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding2, &board, &texture);
        assert_eq!(result2.hand_rank, HandRank::TwoPair(Rank::Nine, Rank::Four));

        // TwoPair(Q4) > TwoPair(94)
        assert_eq!(result1 > result2, true);

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

        let holding_cards = [Card::from("Ts").unwrap(), Card::from("5c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::TwoPair(Rank::Eight, Rank::Six));

        let holding_cards = [Card::from("Ad").unwrap(), Card::from("3c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.hand_rank, HandRank::TwoPair(Rank::Ace, Rank::Eight));
        assert_eq!(result1.hand_rank > result2.hand_rank, false);
        // TwoPair(A8) > TwoPair(86)
        assert_eq!(result1 < result2, true);
    }

    #[test]
    fn two_pairs_with_pocket_pairs() {
        // [Q♦ 5♠], [6♣ 6♦] | T❤ 8♣ 8♠ | K♣ | 4♠	[6♣ 6♦] wins with Pair
        let holding_cards = [Card::from("6c").unwrap(), Card::from("6d").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::TwoPair(Rank::Eight, Rank::Six));
    }

    #[test]
    fn trips_runner_runner() {
        // [T♦ 8♣], [9♠ 5♦] | 2♠ 4♦ T♣ | 9❤ | 9♦	[9♠ 5♦] wins with FullHouse
        let holding_cards = [Card::from("9s").unwrap(), Card::from("5d").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Trips(Rank::Nine));
    }

    #[test]
    fn trips_on_turn_high_card() {
        // [9♣ 3♠], [8♠ 2♦] | T♠ 9❤ J❤ | 9♦ | K❤	[9♣ 3♠] wins with FullHouse
        let holding_cards = [Card::from("9c").unwrap(), Card::from("3s").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Trips(Rank::Nine));
    }

    #[test]
    fn trips_on_turn_low_card() {
        // [9♣ 3♠], [8♠ 2♦] | T♠ 3❤ J❤ | 3♦ | K❤	[9♣ 3♠] wins with FullHouse
        let holding_cards = [Card::from("9c").unwrap(), Card::from("3s").unwrap()];
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Trips(Rank::Three));
    }

    #[test]
    fn trips_on_river_a6() {
        // [5♠ 3♠], [A❤ 6❤ ] | 4♠ A♣ 3❤ | T♣ | A♦	[A❤ 6❤] wins with FullHouse
        let holding_cards = [Card::from("Ah").unwrap(), Card::from("6h").unwrap()];
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

        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Trips(Rank::Ace));
    }

    #[test]
    fn trips_on_river_kj() {
        // [K♠ J♦], [7♦ 6♠] | K♦ 8❤ 5♠ | 4♠ | K♣	[K♠ J♦] wins with FullHouse
        let holding_cards = [Card::from("Ks").unwrap(), Card::from("Jd").unwrap()];
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

        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Trips(Rank::King));
    }

    #[test]
    fn trips_flopped() {
        // [6♣ 6♠], [9♦ 7❤ ] | A❤ 6❤ 9❤ | 4❤ | K♠	[6♣ 6♠] wins with Pair
        let holding_cards = [Card::from("6c").unwrap(), Card::from("6s").unwrap()];
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
        let result1 = HandResult::new(&holding, &board, &texture);

        let holding_cards = [Card::from("9d").unwrap(), Card::from("7h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);

        assert_eq!(result1.hand_rank, HandRank::Trips(Rank::Six));
        assert_eq!(result2.hand_rank, HandRank::Flush(12));
        assert_eq!(result2 > result1, true);
    }

    #[test]
    fn straight_vs_straight() {
        let board_cards = [
            Card::from("Jh").unwrap(),
            Card::from("Ts").unwrap(),
            Card::from("9d").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Straight(Rank::King));

        let holding_cards = [Card::from("7c").unwrap(), Card::from("8c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.hand_rank, HandRank::Straight(Rank::Jack));

        assert_eq!(result1 > result2, true);
    }

    #[test]
    fn straights() {
        let board_cards = [
            Card::from("Jh").unwrap(),
            Card::from("Ts").unwrap(),
            Card::from("9d").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Qc").unwrap(), Card::from("8h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Straight(Rank::Queen));

        let board_cards = [
            Card::from("8h").unwrap(),
            Card::from("Ts").unwrap(),
            Card::from("9d").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("7c").unwrap(), Card::from("Jh").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Straight(Rank::Jack));
    }

    #[test]
    fn straight_five_high() {
        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("2s").unwrap(),
            Card::from("9d").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("8h").unwrap(),
        ];

        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("4c").unwrap(), Card::from("5h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Straight(Rank::Five));
    }

    #[test]
    fn straight_and_flush() {
        let board_cards = [
            Card::from("Jh").unwrap(),
            Card::from("Tc").unwrap(),
            Card::from("9c").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Flush(29));
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::FullHouse(Rank::Ace, Rank::King));
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
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::FullHouse(Rank::Ace, Rank::King));
    }

    #[test]
    fn full_house_paired_board_on_river() {
        // [T♦ 6♦], [A♦ 7♦] | Q♦ 8♣ A♠ | 7❤ | 7♠	[A♦ 7♦] wins with TwoPair
        let board_cards = [
            Card::from("Qd").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("As").unwrap(),
            Card::from("7h").unwrap(),
            Card::from("7s").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let holding_cards = [Card::from("Ad").unwrap(), Card::from("7d").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::FullHouse(Rank::Seven, Rank::Ace));
    }

    // [J❤ 5♠], [9♣ 6❤] | 2❤ 5♦ 2♠ | Q❤ | 5❤ 	[J❤ 5♠] wins with TwoPair
    #[test]
    fn full_house_paired_board_and_river() {
        let board_cards = [
            Card::from("2h").unwrap(),
            Card::from("5d").unwrap(),
            Card::from("2c").unwrap(),
            Card::from("Qh").unwrap(),
            Card::from("5h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let holding_cards = [Card::from("Jh").unwrap(), Card::from("5s").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();

        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::FullHouse(Rank::Five, Rank::Two));
    }

    #[test]
    fn full_house_pockets_and_board_paired_on_river() {
        // [K♣ 2♣], [8❤ 8♠] | 4❤ A♠ Q♣ | 8♦ | Q♦	[8❤ 8♠] wins with TwoPair
        let holding_cards = [Card::from("8h").unwrap(), Card::from("8s").unwrap()];
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

        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::FullHouse(Rank::Eight, Rank::Queen));
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
        let result1 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Quads(Rank::Ace));

        let holding_cards = [Card::from("Tc").unwrap(), Card::from("9c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.hand_rank, HandRank::Quads(Rank::Ace));

        // better Kicker
        assert_eq!(result1 > result2, true);
    }

    #[test]
    fn flush_vs_flush() {
        let board_cards = [
            Card::from("Ah").unwrap(),
            Card::from("6h").unwrap(),
            Card::from("9h").unwrap(),
            Card::from("4h").unwrap(),
            Card::from("Ks").unwrap(),
        ];

        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let holding_cards = [Card::from("Jh").unwrap(), Card::from("7c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);

        let holding_cards = [Card::from("Th").unwrap(), Card::from("7c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result1.hand_rank, HandRank::Flush(21));
        assert_eq!(result2.hand_rank, HandRank::Flush(20));
        assert_eq!(result1 > result2, true);

        // [3♠ 2♣], [A♦ A♠] | 4♠ Q♠ J♠ | 7♣ | 9♠	¯\_(ツ)_/¯ Flush(Spades) vs. Flush(Spades)
        let board_cards = [
            Card::from("4s").unwrap(),
            Card::from("Qs").unwrap(),
            Card::from("Js").unwrap(),
            Card::from("7c").unwrap(),
            Card::from("9s").unwrap(),
        ];

        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let holding_cards = [Card::from("3s").unwrap(), Card::from("2c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);

        let holding_cards = [Card::from("Ad").unwrap(), Card::from("As").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);

        assert_eq!(result1.hand_rank, HandRank::Flush(19));
        assert_eq!(result1.num_ranks, [1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0]);
        assert_eq!(result2.hand_rank, HandRank::Flush(31));
        assert_eq!(result2.num_ranks, [0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 2]);
        assert_eq!(result1 < result2, true);

        // [K♠ 2♣], [Q♠ A♣ ] | 4♠ J♠ A♠ | 7♣ | 9♠	¯\_(ツ)_/¯ Flush(Spades) vs. Flush(Spades)
        let board_cards = [
            Card::from("4s").unwrap(),
            Card::from("Js").unwrap(),
            Card::from("As").unwrap(),
            Card::from("7c").unwrap(),
            Card::from("9s").unwrap(),
        ];

        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        let holding_cards = [Card::from("Ks").unwrap(), Card::from("2c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result1 = HandResult::new(&holding, &board, &texture);

        let holding_cards = [Card::from("Qs").unwrap(), Card::from("Ac").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);

        assert_eq!(result1.hand_rank, HandRank::Flush(32));
        assert_eq!(result1.num_ranks, [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
        assert_eq!(result2.hand_rank, HandRank::Flush(31));
        assert_eq!(result2.num_ranks, [0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 2]);
        assert_eq!(result1 > result2, true);
    }

    #[test]
    fn flush_with_one_on_turn() {
        // [6♣ 6♠], [9♦ 7❤ ] | A❤ 6❤ 9❤ | 4❤ | K♠	[6♣ 6♠] wins with Pair
        let holding_cards = [Card::from("9d").unwrap(), Card::from("7h").unwrap()];
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

        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Flush(12));
    }

    #[test]
    fn straightflush() {
        let board_cards = [
            Card::from("Jh").unwrap(),
            Card::from("Tc").unwrap(),
            Card::from("9c").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("Jc").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::StraightFlush(Rank::King));
    }

    #[test]
    fn royalflush() {
        let board_cards = [
            Card::from("Qc").unwrap(),
            Card::from("Jc").unwrap(),
            Card::from("Tc").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("Jc").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::RoyalFlush);

        let board_cards = [
            Card::from("Qs").unwrap(),
            Card::from("Jc").unwrap(),
            Card::from("Tc").unwrap(),
            Card::from("3c").unwrap(),
            Card::from("Jc").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();

        let holding_cards = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.hand_rank, HandRank::Straight(Rank::Ace));
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<HandRank>(), 16);
        assert_eq!(std::mem::size_of::<HandResult>(), 576);
    }
}
