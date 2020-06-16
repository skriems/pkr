//! This module holds the `HandResult` struct which takes an optional `Board` and computes it's `HandResult`
use crate::board::*;
use crate::card::*;
use crate::holding::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    Trips(Rank),
    Straight(Rank),
    Flush(Suit),
    FullHouse,
    Quads(Rank),
    StraightFlush(Rank),
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
    /// Array of 13 usize for each respective `Rank`
    pub ranks: [[usize; 4]; 13],
    pub num_ranks: [usize; 13],
    pub num_suits: [usize; 4],
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

        HandResult {
            holding,
            board,
            texture,
            ranks,
            num_ranks,
            num_suits,
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

    /// Return the `Suit` for a given amount of Cards
    pub fn suits_on_board(&self, amount: usize) -> Option<Suit> {
        for (idx, num) in self.board.num_suits.iter().enumerate() {
            if *num >= amount {
                return Some(Suit::from(idx));
            }
        }
        None
    }

    /// Return the sum of 5 `Ranks` for a given `Suit`
    pub fn suit_rank(&self, suit: Suit) -> usize {
        let mut rank_sum = 0;
        let mut i = 0;
        for (idx, _num) in self.num_ranks.iter().rev().enumerate() {
            let card = Card::new(Rank::from(12 - idx), suit);
            if self.holding.cards.contains(&card) || self.board.cards().contains(&card) {
                rank_sum += card.rank as usize;
                i += 1;
            }
            if i == 4 {
                break;
            }
        }
        rank_sum
    }

    /// Returns the `Rank` for a given amount of Cards
    pub fn get_rank(&self, amount: usize, skip_first: bool) -> Option<Rank> {
        get_rank(&self.num_ranks, amount, skip_first)
    }

    /// check whether we have a straight
    pub fn has_straight(&self) -> Option<Rank> {
        let mut last_idx = 0;
        let mut connected = 0;

        for (idx, amount) in self.num_ranks.iter().enumerate() {
            if *amount > 0 {
                if idx > 0 && last_idx != idx - 1 {
                    connected = 0;
                }
                connected += 1;
                last_idx = idx;
            }
        }
        if connected == 5 {
            return Some(Rank::from(last_idx));
        }
        None
    }

    pub fn rank_old(&self) -> HandRank {
        if let Some(rank) = self.texture.quads {
            return HandRank::Quads(rank);
        }

        // TODO: similar approach to Straight?
        if let Some(suit) = self.texture.flush {
            return HandRank::Flush(suit);
        }

        if let Some(rank) = self.texture.straight {
            return HandRank::Straight(rank);
        }

        let is_pocket_pair = self.holding.is_pocket_pair();

        if let Some(rank) = self.texture.trips {
            // we can use an arbitrary suit here,
            // because `PartialEq` in `contains`
            // only checks the rank
            let card = Card::new(rank, Suit::Clubs);

            // - Quads
            if self.holding.cards.contains(&card) {
                return HandRank::Quads(rank);
            }

            // - FullHouse
            if let Some(_rank) = self.texture.pair {
                return HandRank::FullHouse;
            } else if is_pocket_pair {
                return HandRank::FullHouse;
            }

            // - Trips High
            return HandRank::Trips(rank);
        }

        if let Some((rank1, rank2)) = self.texture.pairs {
            let card1 = Card::new(rank1, Suit::Clubs);
            let card2 = Card::new(rank2, Suit::Clubs);
            if self.holding.cards.contains(&card1) || self.holding.cards.contains(&card2) {
                return HandRank::FullHouse;
            }
            // TODO: consider pocket pairs?
            return HandRank::TwoPair;
        }

        // let's check if we have a pocket pair and could improve to trips or
        // even Quads. Usefull information even before we know that the
        // self.texture.is_paired
        let mut has_trips: Option<Rank> = None;
        if is_pocket_pair {
            for card in self.board.cards() {
                if self.holding.cards.contains(card) {
                    // we hit our trips! maybe now Quads?
                    if has_trips.is_some() {
                        return HandRank::Quads(card.rank);
                    }
                    has_trips = Some(card.rank);
                }
            }
            // we cannot return Trips here, b/c we have bigger hands to consider below
        }

        if let Some(_pair) = self.texture.pair {
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
                                return HandRank::Trips(*rank);
                            }
                        } else if self.holding.low_card() == &card {
                            // we hit the pair with our low card

                            if self.board.cards().contains(self.holding.high_card()) {
                                return HandRank::FullHouse;
                            } else {
                                // TODO can we have a flush here?
                                return HandRank::Trips(*rank);
                            }
                        }
                    }

                    // - we have a pair on the self.texture and haven't hit it
                    // - we do not have Quads either
                    // - we can still have a FullHouse if `has_trips`
                    if let Some(_rank) = has_trips {
                        return HandRank::FullHouse;
                    }

                    // we can still have TwoPair
                    if is_pocket_pair {
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

        if let Some(suit) = self.texture.flush_with_suited {
            let mut amount_holding: usize = 0;
            for card in self.holding.cards {
                if card.suit == suit {
                    amount_holding += 1;
                }
            }

            if let Some(suit) = self.texture.flush_with_one {
                if amount_holding >= 1 {
                    return HandRank::Flush(suit);
                }
            } else if amount_holding == 2 {
                return HandRank::Flush(suit);
            }
        }

        if let Some(rank) = self.has_straight() {
            return HandRank::Straight(rank);
        }

        if let Some(rank) = has_trips {
            return HandRank::Trips(rank);
        }

        // Now only TwoPair, Pair and HighCards are left
        let mut hits = 0;
        for card in self.board.cards() {
            if self.holding.cards.contains(card) {
                hits += 1;
            }
        }

        if hits == 2 {
            return HandRank::TwoPair;
        } else if hits == 1 || is_pocket_pair {
            return HandRank::Pair;
        } else if let Some(_rank) = self.texture.pair {
            return HandRank::Pair;
        }

        HandRank::HighCard
    }

    pub fn rank(&self) -> HandRank {
        // Straights
        let mut straight_high = 0;
        let mut last_idx = 0;
        let mut connected = 0;

        let mut observed_suits: [usize; 4] = [0, 0, 0, 0];

        // iterating over the num_ranks array
        for (idx, amount) in self.num_ranks.iter().rev().enumerate() {
            if *amount == 0 {
                continue;
            }
            // Straight:
            // whereever we start the first observation of a particular card
            // connected is set to Zero and straight_high to the idx position
            if idx > 0 && last_idx != idx - 1 {
                connected = 0;
                straight_high = idx;
            } else if idx == 1 {
                // special case for King
                straight_high = idx;
            }
            connected += 1;
            last_idx = idx;

            if connected == 5 {
                // Straight or even StraightFlush
                for rank in 7 - straight_high..13 - straight_high {
                    for (suit, num) in self.ranks[rank].iter().enumerate() {
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
                println!("ranks: {:?}", &self.num_ranks);
                println!("obs suits: {:?}", &observed_suits);
                println!("straight_high: {:?}", &straight_high);
                return HandRank::Straight(Rank::from(12 - straight_high));
            }
        }

        HandRank::HighCard
    }
}

impl<'a> PartialEq for HandResult<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.high_cards(5) == other.high_cards(5)
    }
}

/// Determine the order between `self` and `other`. Note, that we only need to do some further
/// processing if the `HandRank` is identical
impl<'a> PartialOrd for HandResult<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_rank = self.rank();
        let other_rank = other.rank();
        if self_rank != other_rank {
            return self_rank.partial_cmp(&other_rank);
        } else {
            match self_rank {
                HandRank::HighCard => self.high_cards(5).partial_cmp(&other.high_cards(5)),
                HandRank::Pair => {
                    let self_pair = self.get_rank(2, false);
                    let other_pair = other.get_rank(2, false);
                    // both have the same pair. So partial_cmp between 3 high_cards
                    if self_pair == other_pair {
                        return self.high_cards(3).partial_cmp(&other.high_cards(3));
                    }
                    // otherwise compare the ranks
                    (self_pair).partial_cmp(&other_pair)
                }
                HandRank::TwoPair => {
                    let self_pair1 = self.get_rank(2, false);
                    let other_pair1 = other.get_rank(2, false);
                    if self_pair1 != other_pair1 {
                        // first pair is not the same, hence we only need to partial_cmp this
                        return self_pair1.partial_cmp(&other_pair1);
                    } else {
                        let self_pair2 = self.get_rank(2, true);
                        let other_pair2 = other.get_rank(2, true);
                        if self_pair2 != other_pair2 {
                            return self_pair2.partial_cmp(&other_pair2);
                        }
                        return self.high_cards(1).partial_cmp(&other.high_cards(1));
                    }
                }
                HandRank::Trips(ref rank1) => match other_rank {
                    HandRank::Trips(ref rank2) => {
                        if rank1 != rank2 {
                            return rank1.partial_cmp(&rank2);
                        }
                        return self.high_cards(2).partial_cmp(&other.high_cards(2));
                    }
                    _ => unreachable!(),
                },
                HandRank::Flush(ref suit) => {
                    let self_suit_rank = self.suit_rank(*suit);
                    let other_suit_rank = other.suit_rank(*suit);
                    return self_suit_rank.partial_cmp(&other_suit_rank);
                }
                HandRank::FullHouse => {
                    let self_trips = self.get_rank(3, false);
                    let other_trips = other.get_rank(3, false);
                    if self_trips != other_trips {
                        return self_trips.partial_cmp(&other_trips);
                    } else {
                        let self_pair = self.get_rank(2, false);
                        let other_pair = other.get_rank(2, false);
                        return self_pair.partial_cmp(&other_pair);
                    }
                }
                HandRank::Quads(ref rank1) => match other_rank {
                    HandRank::Quads(ref rank2) => {
                        if rank1 != rank2 {
                            return rank1.partial_cmp(&rank2);
                        }
                        return self.high_cards(1).partial_cmp(&other.high_cards(1));
                    }
                    _ => unreachable!(),
                },
                // TODO StraightFlush
                // TODO RoyalFlush
                _ => self_rank.partial_cmp(&other_rank),
            }
        }
    }
}

/// Returns the `Rank` for a given amount of Cards. Use `skip_first` to get the i.e. 2nd Pair
pub fn get_rank(ranks: &[usize; 13], amount: usize, skip_first: bool) -> Option<Rank> {
    let mut found = false;
    for (idx, num) in ranks.iter().rev().enumerate() {
        if *num == amount {
            if skip_first && !found {
                found = true;
                continue;
            } else {
                return Some(Rank::from(12 - idx));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank() {
        assert_eq!(HandRank::HighCard < HandRank::Pair, true);
        assert_eq!(HandRank::Pair < HandRank::TwoPair, true);
        assert_eq!(HandRank::TwoPair < HandRank::Trips(Rank::Two), true);
        assert_eq!(
            HandRank::Trips(Rank::Three) < HandRank::Trips(Rank::Jack),
            true
        );
        assert_eq!(
            HandRank::Trips(Rank::Jack) < HandRank::Straight(Rank::Jack),
            true
        );
        assert_eq!(
            HandRank::Straight(Rank::Jack) < HandRank::Straight(Rank::Queen),
            true
        );
        assert_eq!(
            HandRank::Straight(Rank::Jack) < HandRank::Flush(Suit::Clubs),
            true
        );
        assert_eq!(HandRank::Flush(Suit::Clubs) < HandRank::FullHouse, true);
        assert_eq!(HandRank::FullHouse < HandRank::Quads(Rank::Two), true);
        assert_eq!(
            HandRank::Quads(Rank::Two) < HandRank::Quads(Rank::Three),
            true
        );
        assert_eq!(
            HandRank::Quads(Rank::Four) < HandRank::StraightFlush(Rank::Ace),
            true
        );
        assert_eq!(
            HandRank::StraightFlush(Rank::Ace) < HandRank::RoyalFlush,
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
        assert_eq!(result.rank(), HandRank::Pair);
        assert_eq!(result.num_ranks, [1, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 1, 1]);
        assert_eq!(result.get_rank(2, false), Some(Rank::Eight));
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
        assert_eq!(result.rank(), HandRank::HighCard);

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
        assert_eq!(result.rank(), HandRank::HighCard);

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

        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.suits_on_board(3).unwrap(), Suit::Diamonds);
        assert_eq!(result.rank(), HandRank::HighCard);
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

        assert_eq!(result1.rank(), HandRank::HighCard);
        assert_eq!(result2.rank(), HandRank::HighCard);
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Pair);
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
        assert_eq!(result1.rank(), HandRank::Pair);

        let holding_cards = [Card::from("6s").unwrap(), Card::from("4h").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);

        println!("get_rank: {:?}", get_rank(&result1.num_ranks, 2, false));
        println!("get_rank: {:?}", get_rank(&result2.num_ranks, 2, false));
        println!("ranks: {:?}", result1.ranks);
        println!("ranks: {:?}", result2.ranks);
        println!("high_cards: {:?}", result1.high_cards(3));
        println!("high_cards: {:?}", result2.high_cards(3));
        assert_eq!(result2.rank(), HandRank::Pair);
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
        assert_eq!(result.rank(), HandRank::TwoPair);
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
        assert_eq!(result.rank(), HandRank::TwoPair);
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
        assert_eq!(result1.rank(), HandRank::TwoPair);
        assert_eq!(result1.get_rank(2, false), Some(Rank::Six));
        assert_eq!(result1.num_ranks, [0, 1, 2, 0, 2, 1, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(result1.get_rank(2, true), Some(Rank::Four));

        let holding_cards = [Card::from("9s").unwrap(), Card::from("3d").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.rank(), HandRank::TwoPair);
        assert_eq!(result2.get_rank(2, false), Some(Rank::Six));
        assert_eq!(result2.get_rank(2, true), Some(Rank::Three));

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
        assert_eq!(result1.rank(), HandRank::TwoPair);

        let holding_cards = [Card::from("9d").unwrap(), Card::from("3h").unwrap()];
        let holding2 = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding2, &board, &texture);
        println!("{:#?}", board);
        println!("{:#?}", texture);
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result2.rank(), HandRank::TwoPair);

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
        assert_eq!(result1.rank(), HandRank::TwoPair);

        let holding_cards = [Card::from("Ad").unwrap(), Card::from("3c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.rank(), HandRank::TwoPair);
        assert_eq!(result1.rank() > result2.rank(), false);
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::TwoPair);
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Trips(Rank::Nine));
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Trips(Rank::Nine));
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Trips(Rank::Three));
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Trips(Rank::Ace));
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Trips(Rank::King));
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

        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result1.rank(), HandRank::Trips(Rank::Six));
        assert_eq!(result2.rank(), HandRank::Flush(Suit::Hearts));
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
        assert_eq!(result1.rank(), HandRank::Straight(Rank::King));

        let holding_cards = [Card::from("7c").unwrap(), Card::from("8c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.rank(), HandRank::Straight(Rank::Jack));

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
        assert_eq!(result1.rank(), HandRank::Straight(Rank::Queen));

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
        assert_eq!(result1.rank(), HandRank::Straight(Rank::Jack));
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
        assert_eq!(result.rank(), HandRank::Flush(Suit::Clubs));
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
        let texture = board.texture();
        let result = HandResult::new(&holding, &board, &texture);
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::FullHouse);
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
        assert_eq!(result1.rank(), HandRank::Quads(Rank::Ace));

        let holding_cards = [Card::from("Tc").unwrap(), Card::from("9c").unwrap()];
        let holding = Holding::new(&holding_cards).unwrap();
        let result2 = HandResult::new(&holding, &board, &texture);
        assert_eq!(result2.rank(), HandRank::Quads(Rank::Ace));

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
        assert_eq!(result1.rank(), HandRank::Flush(Suit::Hearts));
        assert_eq!(result1.suit_rank(Suit::Hearts), 39);
        assert_eq!(result2.rank(), HandRank::Flush(Suit::Hearts));
        assert_eq!(result2.suit_rank(Suit::Hearts), 38);
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

        assert_eq!(result1.rank(), HandRank::Flush(Suit::Spades));
        assert_eq!(result1.num_ranks, [1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0]);
        assert_eq!(result1.suit_rank(Suit::Hearts), 31);
        assert_eq!(result2.rank(), HandRank::Flush(Suit::Spades));
        assert_eq!(result2.num_ranks, [0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 2]);
        assert_eq!(result2.suit_rank(Suit::Hearts), 38);
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

        assert_eq!(result1.rank(), HandRank::Flush(Suit::Spades));
        assert_eq!(result1.num_ranks, [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
        assert_eq!(result1.suit_rank(Suit::Hearts), 39);
        assert_eq!(result2.rank(), HandRank::Flush(Suit::Spades));
        assert_eq!(result2.num_ranks, [0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 2]);
        assert_eq!(result2.suit_rank(Suit::Hearts), 38);
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
        println!("{:#?}", board);
        println!("{:#?}", board.texture());
        println!("board.pairs: {:#?}", board.pairs());
        assert_eq!(result.rank(), HandRank::Flush(Suit::Hearts));
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
        assert_eq!(result.rank(), HandRank::StraightFlush(Rank::King));
        // assert_eq!(result.rank(), HandRank::StraightFlush(Rank::King));
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<HandRank>(), 2);
        assert_eq!(std::mem::size_of::<HandResult>(), 576);
    }
}
