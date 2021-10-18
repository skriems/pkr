use crate::card::*;
use crate::hand_rank::*;

pub fn setup_array(cards: &[&Card]) -> ([[usize; 4]; 13], [usize; 13], [usize; 4]) {
    let mut ranks = [
        // clubs, spades, hearts, diamonds
        [0, 0, 0, 0], // Two
        [0, 0, 0, 0], // Three
        [0, 0, 0, 0], // Four
        [0, 0, 0, 0], // Five
        [0, 0, 0, 0], // Six
        [0, 0, 0, 0], // Seven
        [0, 0, 0, 0], // Eight
        [0, 0, 0, 0], // Nine
        [0, 0, 0, 0], // Ten
        [0, 0, 0, 0], // Jack
        [0, 0, 0, 0], // Queen
        [0, 0, 0, 0], // King
        [0, 0, 0, 0], // Ace
    ];

    // number of cards from Two -> Ace
    let mut num_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    // number of suites from Clubs to Diamonds
    let mut num_suits = [0, 0, 0, 0];

    for card in cards {
        let rank = card.rank as usize;
        let suit = card.suit as usize;
        ranks[rank][suit] = 1;
        num_ranks[rank] += 1;
        num_suits[suit] += 1;
    }

    (ranks, num_ranks, num_suits)
}

/// Hand
#[derive(Debug)]
pub struct Hand {
    pub ranks: [[usize; 4]; 13],
    pub num_ranks: [usize; 13],
    pub num_suits: [usize; 4],
}

impl Hand {
    pub fn new(holdings: &[Card], community_cards: &[Card]) -> Self {
        let mut ranks = [
            // clubs, spades, hearts, diamonds
            [0, 0, 0, 0], // Two
            [0, 0, 0, 0], // Three
            [0, 0, 0, 0], // Four
            [0, 0, 0, 0], // Five
            [0, 0, 0, 0], // Six
            [0, 0, 0, 0], // Seven
            [0, 0, 0, 0], // Eight
            [0, 0, 0, 0], // Nine
            [0, 0, 0, 0], // Ten
            [0, 0, 0, 0], // Jack
            [0, 0, 0, 0], // Queen
            [0, 0, 0, 0], // King
            [0, 0, 0, 0], // Ace
        ];

        // number of cards from Two -> Ace
        let mut num_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        // number of suites from Clubs to Diamonds
        let mut num_suits = [0, 0, 0, 0];

        for card in holdings {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        for card in community_cards {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        Hand {
            ranks,
            num_ranks,
            num_suits,
        }
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
    pub fn rank(&self, other: &[&Card]) -> HandRank {
        let (ranks, num_ranks, num_suits) = setup_array(other);

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
        let mut high_card_rank: Option<Rank> = None;

        // NOTE that we're iterating over the reversed `num_ranks` array!! Hence `card_rank` (index) of
        // 0 represents an Ace instead of a Two
        for (card_rank, amount) in num_ranks.iter().rev().enumerate() {
            let n = *amount;
            let num = n + self.num_ranks[12 - card_rank];

            if num == 0 {
                continue;
            }

            if num == 1 && high_card_rank.is_none() {
                high_card_rank = Some(Rank::from(12 - card_rank));
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
                for (suit, n) in ranks[rank].iter().enumerate() {
                    let num = n + self.ranks[rank][suit];
                    if num > 0 {
                        if rank > 0 && ranks[prev_rank][suit] == 1
                            || self.ranks[prev_rank][suit] == 1
                        {
                            observed_suits[suit] += num;
                        } else if rank == 0 && ranks[prev_rank][suit] == 1
                            || self.ranks[prev_rank][suit] == 1
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

        for (suit, n) in num_suits.iter().enumerate() {
            let num = n + self.num_suits[suit];
            if num == 5 {
                return HandRank::Flush(suit_rank(&ranks, suit) + suit_rank(&self.ranks, suit));
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

        if let Some(rank) = high_card_rank {
            return HandRank::HighCard(rank);
        }
        HandRank::HighCard(Rank::Two)
    }
}

/// return the sum of 5 `Ranks` for a given `Suit`
/// TODO this could be incorporated into the big fat loop in `ranks`
pub fn suit_rank(ranks: &[[usize; 4]; 13], suit: usize) -> usize {
    let mut rank_sum = 0;
    let mut counted = 0;
    for (rank, suits) in ranks.iter().rev().enumerate() {
        if suits[suit] > 0 {
            rank_sum += 12 - rank;
            counted += 1;
        }

        if counted == 5 {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::setup_arrays;

//     #[test]
//     fn internal_ranks() {
//         // [6♠ 4❤], [K♦ 7❤] | J♦ A♠ 8♦ | 8♣ | 2❤	¯\_(ツ)_/¯ HighCard vs. HighCard
//         let cards = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
//         let holding = vec![&cards[..]];
//         let community_cards = [
//             Card::from("Jd").unwrap(),
//             Card::from("As").unwrap(),
//             Card::from("8c").unwrap(),
//             Card::from("8c").unwrap(),
//             Card::from("2h").unwrap(),
//         ];
//         let combo = vec![];

//         let (ranks, num_ranks, num_suits) = setup_arrays(&holding, &community_cards, &combo);
//         let result = Hand::bare(&ranks[0], &num_ranks[0], &num_suits[0]);

//         assert_eq!(result.hand_rank, HandRank::Pair(Rank::Eight));
//         assert_eq!(result.num_ranks, &[1, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 1, 1]);
//     }

//     #[test]
//     fn test_suit_rank() {
//         let ranks = [
//             // clubs, spades, hearts, diamonds
//             [0, 0, 0, 0], // Two
//             [0, 0, 0, 0], // Three
//             [0, 0, 0, 0], // Four
//             [0, 0, 0, 0], // Five
//             [0, 0, 0, 0], // Six
//             [0, 0, 0, 0], // Seven
//             [0, 0, 0, 0], // Eight
//             [0, 0, 1, 0], // Nine
//             [0, 0, 1, 0], // Ten
//             [0, 0, 1, 0], // Jack
//             [0, 0, 1, 0], // Queen
//             [0, 0, 1, 0], // King
//             [0, 0, 1, 0], // Ace
//         ];
//         // assert that we only count the 5 highest cards
//         // 12 + 11 + 10 + 9 + 8
//         assert_eq!(suit_rank(&ranks, 2), 50);
//     }

//     #[test]
//     fn mem() {
//         assert_eq!(std::mem::size_of::<Hand>(), 40);
//     }
// }
