use crate::card::*;
use crate::raw_data::*;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandRank {
    HighCard,
    Pair(Rank),
    TwoPair(Rank, Rank),
    Trips(Rank),
    Straight(Rank),
    Flush(Suit),
    FullHouse(Rank, Rank),
    Quads(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

// impl PartialOrd for HandRank {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match (self, other) {
//             (HandRank::TwoPair(ref rank, ref _x), HandRank::TwoPair(ref other_rank, ref _y))
//             | (
//                 HandRank::FullHouse(ref rank, ref _x),
//                 HandRank::FullHouse(ref other_rank, ref _y),
//             ) => rank.partial_cmp(&other_rank),
//             (_, _) => self.partial_cmp(&other),
//         }
//     }
// }

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandRank::HighCard => write!(f, "HighCard"),
            HandRank::Pair(ref _r) => write!(f, "Pair"),
            HandRank::TwoPair(ref _r, ref _n) => write!(f, "TwoPair"),
            HandRank::Trips(ref _r) => write!(f, "Trips"),
            HandRank::Straight(ref _r) => write!(f, "Straight"),
            HandRank::Flush(ref _r) => write!(f, "Flush"),
            HandRank::FullHouse(ref _r, ref _n) => write!(f, "FullHouse"),
            HandRank::Quads(ref _r) => write!(f, "Quads"),
            HandRank::StraightFlush(ref _r) => write!(f, "StraitFlush"),
            HandRank::RoyalFlush => write!(f, "RoyalFlush"),
        }
    }
}

impl From<&HandRank> for usize {
    fn from(rank: &HandRank) -> Self {
        match rank {
            HandRank::HighCard => 0,
            HandRank::Pair(ref _r) => 1,
            HandRank::TwoPair(ref _r, ref _n) => 2,
            HandRank::Trips(ref _r) => 3,
            HandRank::Straight(ref _r) => 4,
            HandRank::Flush(ref _r) => 5,
            HandRank::FullHouse(ref _r, ref _n) => 6,
            HandRank::Quads(ref _r) => 7,
            HandRank::StraightFlush(ref _r) => 8,
            HandRank::RoyalFlush => 9,
        }
    }
}

impl From<usize> for HandRank {
    fn from(n: usize) -> Self {
        match n {
            0 => HandRank::HighCard,
            1 => HandRank::Pair(Rank::Ace),
            2 => HandRank::TwoPair(Rank::Ace, Rank::Ace),
            3 => HandRank::Trips(Rank::Ace),
            4 => HandRank::Straight(Rank::Ace),
            5 => HandRank::Flush(Suit::Spades),
            6 => HandRank::FullHouse(Rank::Ace, Rank::King),
            7 => HandRank::Quads(Rank::Ace),
            8 => HandRank::StraightFlush(Rank::Ace),
            9 => HandRank::RoyalFlush,
            _ => unreachable!(),
        }
    }
}

/**
 * Returns the computed `HandRank`.
 *
 * we're iterating over the `own.num_ranks` array in reverse order, the Card::Ranks get reversed
 * too. Hence
 *
 * enum Rank {
 *           norm_rank       card_rank
 *   Two,        0      ->      12
 *   Three,      1      ->      11
 *   Four,       2      ->      10
 *   Five,       3      ->       9
 *   Six,        4      ->       8
 *   Seven,      5      ->       7
 *   Eight,      6      ->       6
 *   Nine,       7      ->       5
 *   Ten,        8      ->       4
 *   Jack,       9      ->       3
 *   Queen,     10      ->       2
 *   King,      11      ->       1
 *   Ace,       12      ->       0
 */
pub fn rank(own: &RawData, other: &RawData) -> HandRank {
    // what was the last card rank
    let mut last_rank = 0;
    // how many connected cards do we have
    let mut connected = 0;
    let mut has_straight = false;
    // highest card of a straight
    let mut straight_high = 0;
    // does the straight have an ace?
    let mut straight_ace = false;

    let mut pairs: [Option<Rank>; 2] = [None, None];
    let mut trips: Option<Rank> = None;
    let mut quads: Option<Rank> = None;
    let mut full_house: bool = false;

    // NOTE that we're iterating over the reversed `num_ranks` array!! Hence `card_rank` (index) of
    // 0 represents an Ace instead of a Two
    for (card_rank, amount) in own.num_ranks.iter().rev().enumerate() {
        let num = *amount + other.num_ranks[12 - card_rank]; // 12 - card_rank b/c rev() !!

        if num == 0 {
            continue;
        }

        if num == 2 {
            if trips.is_some() {
                // if we already have trips, we now have a full house
                full_house = true;
            }

            if !pairs[1].is_some() {
                if let Some(_rank) = pairs[0] {
                    pairs[1] = Some(Rank::from(12 - card_rank));
                } else {
                    pairs[0] = Some(Rank::from(12 - card_rank));
                }
            }
        }

        if num == 3 {
            trips = Some(Rank::from(12 - card_rank));
            if pairs[0].is_some() || pairs[1].is_some() {
                full_house = true;
            }
        }

        if num == 4 {
            quads = Some(Rank::from(12 - card_rank));
        }

        // The next block of code determines, if hero has a straight. It increments `connected` if
        // we've seen `last_rank` or resets it to 0 if not.
        // A2345 straights mark a special case since the algorythm only detects four consecutive
        // cards (2345), so we need `straight_ace` as a helper variable.
        if num > 0 && !has_straight {
            if card_rank > 0 && last_rank != card_rank - 1 {
                connected = 0;
                straight_high = card_rank;
            } else if card_rank == 0 {
                straight_ace = true;
                straight_high = card_rank;
            } else if card_rank == 1 && !straight_ace {
                // since we're initializing `last_rank` to 0 (Ace) the code jumbs over the first if
                // statement. Hence, for an KQJT9 straight, we need this branch...
                straight_high = card_rank;
            }
            // increment
            connected += 1;
            last_rank = card_rank;
        }

        // we either have 5 connected cards or a A2345 straight
        if connected == 5 || connected == 4 && straight_high == 9 && straight_ace {
            has_straight = true;
        }
    }

    // If the code above has determined a straight, check if we have a StraightFlush or even
    // RoyalFlush. We're iterating over the `[straight_high - 5..straight_high]` cards, check if
    // have seen the `prev_rank` (same conecpt as `last_rank` really) in the same suit
    if has_straight {
        let mut norm_rank = 12 - straight_high;
        if norm_rank < 5 {
            norm_rank = 5;
        }
        let start = norm_rank - 4;

        // same conecpt like `last_rank`
        let mut prev_rank = start;
        let mut observed_suits: [usize; 4] = [0, 0, 0, 0];

        // norm_rank + 1 b/c upper bound is exclusive
        for rank in start..norm_rank + 1 {
            for (suit, n) in other.ranks[rank].iter().enumerate() {
                let num = n + own.ranks[rank][suit];
                if num > 0 {
                    if rank > 0 && other.ranks[prev_rank][suit] == 1
                        || own.ranks[prev_rank][suit] == 1
                    {
                        observed_suits[suit] += num;
                    } else if rank == 0 && other.ranks[prev_rank][suit] == 1
                        || own.ranks[prev_rank][suit] == 1
                    {
                        observed_suits[suit] += num;
                    }
                }
            }
            prev_rank = rank;
        }
        for num_suit in observed_suits.iter() {
            if *num_suit == 5 {
                // TODO here we need to check if the street is composed of 5 cards of the same suit
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

    for (suit, n) in own.num_suits.iter().enumerate() {
        let num = n + other.num_suits[suit];
        if num == 5 {
            return HandRank::Flush(Suit::from(suit));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_rank_ordering() {
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
    fn hand_rank_equlity() {
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
            HandRank::Flush(Suit::Hearts) == HandRank::Flush(Suit::Hearts),
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
    fn mem() {
        assert_eq!(std::mem::size_of::<HandRank>(), 3);
    }
}
