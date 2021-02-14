use crate::card::*;
use crate::hand_rank::*;
use std::cmp::Ordering;

/// Hand
#[derive(Debug)]
pub struct Hand<'a> {
    pub ranks: &'a [[usize; 4]; 13],
    pub num_ranks: &'a [usize; 13],
    pub num_suits: &'a [usize; 4],
    pub hand_rank: HandRank,
}

impl<'a> Hand<'a> {
    pub fn bare(
        ranks: &'a [[usize; 4]; 13],
        num_ranks: &'a [usize; 13],
        num_suits: &'a [usize; 4],
    ) -> Self {
        let hand_rank = rank(&ranks, &num_ranks, &num_suits);

        Hand {
            ranks,
            num_ranks,
            num_suits,
            hand_rank,
        }
    }

    /// return the sum of `Ranks` for a given `amount` of HighCards
    pub fn high_cards(&self, amount: usize) -> usize {
        high_cards(*self.num_ranks, amount)
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.high_cards(5) == other.high_cards(5)
    }
}

impl<'a> PartialOrd for Hand<'a> {
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

/// return the sum of 5 `Ranks` for a given `Suit`
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

/// return the sum of `Ranks` for a given `amount` of HighCards
pub fn high_cards(num_ranks: [usize; 13], amount: usize) -> usize {
    let mut rank_sum = 0;
    let mut i = 0;
    for (rank, num) in num_ranks.iter().rev().enumerate() {
        if *num == 1 {
            rank_sum += 12 - rank;
            i += num;
        }
        if i == amount {
            break;
        }
    }
    rank_sum
}

/**
 * Returns the computed `HandRank`.
 *
 * since we're iterating over the `num_ranks` array in reverse order, the Card::Ranks get reversed
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
fn rank(ranks: &[[usize; 4]; 13], num_ranks: &[usize; 13], num_suits: &[usize; 4]) -> HandRank {
    // what was the last card rank
    let mut last_rank = 0;
    // how many connected cards do we have
    let mut connected = 0;
    let mut has_straight = false;
    // highest card of a straight
    let mut straight_high = 0;
    // does the straight have an ace?
    let mut straight_ace = false;

    let mut observed_suits: [usize; 4] = [0, 0, 0, 0];

    let mut pairs: [Option<Rank>; 2] = [None, None];
    let mut trips: Option<Rank> = None;
    let mut quads: Option<Rank> = None;
    let mut full_house: bool = false;

    // NOTE that we're iterating over the reversed `num_ranks` array!! Hence `card_rank` (index) of
    // 0 represents an Ace instead of a Two
    for (card_rank, amount) in num_ranks.iter().rev().enumerate() {
        let num = *amount;
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
        let start = norm_rank - 5;

        // same conecpt like `last_rank`
        let mut prev_rank = start;
        // norm_rank + 1 b/c upper bound is exclusive
        for rank in start..norm_rank + 1 {
            for (suit, num) in ranks[rank].iter().enumerate() {
                if rank > 0 && prev_rank == rank - 1 {
                    observed_suits[suit] += num;
                } else if rank == 0 {
                    observed_suits[suit] += num;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup_arrays;

    #[test]
    fn internal_ranks() {
        // [6♠ 4❤], [K♦ 7❤] | J♦ A♠ 8♦ | 8♣ | 2❤	¯\_(ツ)_/¯ HighCard vs. HighCard
        let cards = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
        let holding = vec![&cards[..]];
        let community_cards = [
            Card::from("Jd").unwrap(),
            Card::from("As").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let combo = vec![];

        let (ranks, num_ranks, num_suits) = setup_arrays(&holding, &community_cards, &combo);
        let result = Hand::bare(&ranks[0], &num_ranks[0], &num_suits[0]);

        assert_eq!(result.hand_rank, HandRank::Pair(Rank::Eight));
        assert_eq!(result.num_ranks, &[1, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 1, 1]);
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Hand>(), 40);
    }
}
