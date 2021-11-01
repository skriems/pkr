use crate::card::*;
use crate::hand_rank::*;
use crate::raw_data::*;

/// Hand
#[derive(Debug)]
pub struct Hand<'a> {
    pub own: &'a RawData,
    pub combo: RawData,
    pub rank: HandRank,
}

impl<'a> Hand<'a> {
    pub fn new(own: &'a RawData, combo: &'a Vec<&Card>) -> Self {
        let combo = RawData::new(combo);
        let rank = rank(&own, &combo);
        Hand { own, combo, rank }
    }

    /// return the sum of 5 `Ranks` for a given `Suit`
    pub fn suit_rank(&self, suit: usize) -> usize {
        let mut rank_sum = 0;
        let mut counted = 0;
        for (rank, suits) in self.own.ranks.iter().rev().enumerate() {
            if suits[suit] > 0 {
                rank_sum += 12 - rank;
                counted += 1;
            }
            if self.combo.ranks[12 - rank][suit] > 0 {
                rank_sum += 12 - rank;
                counted += 1;
            }

            if counted == 5 {
                break;
            }
        }
        rank_sum
    }

    /// return the sum of `Ranks` for a given `amount` Cards
    pub fn high_cards(&self, amount: usize) -> usize {
        let mut rank_sum = 0;
        let mut i = 0;
        for (rank, num) in self.own.num_ranks.iter().rev().enumerate() {
            if *num == 1 {
                rank_sum += 12 - rank;
                rank_sum += self.combo.num_ranks[12 - rank];
                i += num;
            }
            if i == amount {
                break;
            }
        }
        rank_sum
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self.rank {
            HandRank::HighCard => self.high_cards(5).eq(&other.high_cards(5)),
            HandRank::Flush(ref suit) => self
                .suit_rank(*suit as usize)
                .eq(&other.suit_rank(*suit as usize)),
            _ => self.rank.eq(&other.rank),
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank == other.rank {
            match self.rank {
                HandRank::HighCard => self.high_cards(5).partial_cmp(&other.high_cards(5)),
                HandRank::Flush(ref suit) => self
                    .suit_rank(*suit as usize)
                    .partial_cmp(&other.suit_rank(*suit as usize)),
                _ => self.rank.partial_cmp(&other.rank),
            }
        } else {
            self.rank.partial_cmp(&other.rank)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_ranks() {
        // [6♠ 4❤], [K♦ 7❤] | J♦ A♠ 8♦ | 8♣ | 2❤	¯\_(ツ)_/¯ HighCard vs. HighCard
        let community_cards = [
            Card::from("Jd").unwrap(),
            Card::from("As").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let holdings = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
        let chain = holdings.iter().chain(community_cards.iter());
        let combo = vec![];

        let raw_data = RawData::from_chain(chain);
        let hand = Hand::new(&raw_data, &combo);

        assert_eq!(hand.rank, HandRank::Pair(Rank::Eight));
    }

    #[test]
    fn suit_rank() {
        let combo: Vec<&Card> = vec![];
        let community_cards = [
            Card::from("Jd").unwrap(),
            Card::from("As").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let holdings = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
        let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

        let hand = Hand::new(&raw_cards, &combo);

        // test
        assert_eq!(
            hand.suit_rank(Suit::Clubs as usize),
            Card::from("8c").unwrap().rank as usize
        );

        let community_cards = [
            Card::from("Jd").unwrap(),
            Card::from("Ad").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let holdings = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
        let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

        let hand = Hand::new(&raw_cards, &combo);

        // test
        assert_eq!(
            hand.suit_rank(Suit::Diamonds as usize),
            Card::from("Jd").unwrap().rank as usize
                + Card::from("Ad").unwrap().rank as usize
                + Card::from("8d").unwrap().rank as usize
                + Card::from("Kd").unwrap().rank as usize
        );

        let community_cards = [
            Card::from("Jc").unwrap(),
            Card::from("Ac").unwrap(),
            Card::from("8d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("2h").unwrap(),
        ];
        let holdings = [Card::from("Kc").unwrap(), Card::from("7h").unwrap()];
        let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

        let hand = Hand::new(&raw_cards, &combo);

        // test
        assert_eq!(
            hand.suit_rank(Suit::Clubs as usize),
            Card::from("Jc").unwrap().rank as usize
                + Card::from("Ac").unwrap().rank as usize
                + Card::from("8c").unwrap().rank as usize
                + Card::from("Kc").unwrap().rank as usize
        );
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Hand>(), 568);
    }
}
