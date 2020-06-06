use crate::card::*;
use crate::error::{Error, Result};
use crate::Beats;

/// A Players Holding Cards
#[derive(Debug, PartialOrd)]
pub struct Holding<'a> {
    cards: &'a [Card],
}

impl<'a> Holding<'a> {
    pub fn new(cards: &'a [Card]) -> Result<Self> {
        if cards.len() != 2 {
            return Err(Error::InvalidHolding);
        }

        let first = &cards[0];
        let second = &cards[1];

        if first.rank == second.rank && first.suit == second.suit {
            return Err(Error::InvalidHolding);
        };

        Ok(Holding { cards })
    }

    pub fn high_card(&self) -> &Card {
        if self.cards[0].beats(&self.cards[1]) || self.cards[0].pairs(&self.cards[1]) {
            &self.cards[0]
        } else {
            &self.cards[1]
        }
    }

    pub fn low_card(&self) -> &Card {
        if self.cards[0].beats(&self.cards[1]) || self.cards[0].pairs(&self.cards[1]) {
            &self.cards[1]
        } else {
            &self.cards[0]
        }
    }

    pub fn is_pocket_pair(&self) -> bool {
        self.cards[0].pairs(&self.cards[1])
    }

    // pub fn from(expr: &'a str) -> Result<Self> {
    //     if expr.len() != 4 {
    //         return Err(Error::ParseError);
    //     }

    //     let first = Card::from(&expr[..2])?;
    //     let second = Card::from(&expr[2..4])?;
    //     let cards = [first, second];
    //     Holding::new(cards)
    // }
}

impl<'a> Beats for Holding<'a> {
    fn beats(&self, other: &Self) -> bool {
        // pocket pairs: check them late because they are rather rare
        if self.is_pocket_pair() {
            if !other.is_pocket_pair() {
                return true;
            }
            return self.high_card().beats(other.high_card());
        }

        if other.is_pocket_pair() && !self.is_pocket_pair() {
            return false;
        };

        // higher high_card
        if self.high_card().beats(other.high_card()) {
            return true;
        }

        // having a better kicker
        if self.high_card().pairs(other.high_card()) {
            return self.low_card().beats(other.low_card());
        }

        // i.e. same hand
        return false;
    }

    fn pairs(&self, other: &Self) -> bool {
        // AK vs AK
        self.cards[0].pairs(&other.cards[0]) && self.cards[1].pairs(&other.cards[1]) ||
        // KA vs AK
        self.cards[1].pairs(&other.cards[0]) && self.cards[0].pairs(&other.cards[1])  ||
        // AK vs KA
        self.cards[0].pairs(&other.cards[1]) && self.cards[1].pairs(&other.cards[0])  ||
        // KA vs KA
        self.cards[1].pairs(&other.cards[1]) && self.cards[0].pairs(&other.cards[0])
    }
}

impl<'a> PartialEq for Holding<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards[0] == other.cards[0] && self.cards[1] == other.cards[1]
            || self.cards[0] == other.cards[1] && self.cards[1] == other.cards[0]
            || self.cards[1] == other.cards[0] && self.cards[0] == other.cards[1]
            || self.cards[1] == other.cards[1] && self.cards[0] == other.cards[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // same Rank, different Suit
        let cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        ];
        assert!(Holding::new(&cards).is_ok());
        assert_eq!(Holding::new(&cards).unwrap(), Holding { cards: &cards });

        // different Rank, suited
        let cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        assert!(Holding::new(&cards).is_ok());
        assert_eq!(Holding::new(&cards).unwrap(), Holding { cards: &cards });

        // same Rank, suited
        let cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        assert!(Holding::new(&cards).is_err());
    }

    // #[test]
    // fn from_expr() {
    //     // different Rank, suited
    //     assert!(Holding::from("AsKs").is_ok());
    //     // same cards
    //     assert!(Holding::from("AsAs").is_err());
    //     // same Rank, different Suit
    //     assert!(Holding::from("AsAd").is_ok());
    //     // different Rank, different Suit
    //     assert!(Holding::from("AsKc").is_ok());
    // }

    #[test]
    fn partial_eq() {
        // Aces
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];

        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert_eq!(first, second);

        // AK's
        let first_cards = [
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Hearts),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn high_card() {
        // AK -> &A
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let holding = Holding::new(&first_cards).unwrap();
        assert_eq!(holding.high_card(), &Card::new(Rank::Ace, Suit::Clubs));
    }

    #[test]
    fn low_card() {
        // AK -> &K
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let holding = Holding::new(&first_cards).unwrap();
        assert_eq!(holding.low_card(), &Card::new(Rank::King, Suit::Clubs));
    }

    #[test]
    fn beats() {
        // two overcards: AK vs QJ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // two overcards: KA vs QJ
        let first_cards = [
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // two overcards: AK vs JQ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: AK vs AQ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: KA vs AQ
        let first_cards = [
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: AK vs QA
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: KA vs QA
        let first_cards = [
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // AK vs AK
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // KA vs AK
        let first_cards = [
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // AK vs KA
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // KA vs KA
        let first_cards = [
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // AJ vs KQ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // J7 vs T8
        let first_cards = [
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // 55 vs AK
        let first_cards = [
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::King, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // 55 vs 44
        let first_cards = [
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // 55 vs 55
        let first_cards = [
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // AK vs 55
        let first_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);
    }

    #[test]
    fn pairs() {
        // AK vs AK
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // KA vs AK
        let first_cards = [
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // AK vs KA
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // KA vs KA
        let first_cards = [
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // AK vs AQ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), false);

        // 87 vs 87
        let first_cards = [
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Seven, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // 55 vs 55
        let first_cards = [
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let second_cards = [
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), false);
    }

    #[test]
    fn looses() {
        // 44 vs 55
        let first_cards = [
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.looses(&other), true);
    }

    #[test]
    fn partial_ne() {
        // AK vs AQ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Diamonds),
        ];

        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn partial_ord() {
        // Aces vs Kings
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert!(first > second);

        // AK vs AQ
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert!(first > second);

        // AQ vs AK
        let first_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
        ];
        let second_cards = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert!(first < second);
    }

    // #[test]
    // fn test_slice() {
    //     let cards = [
    //         Card::new(Rank::Ace, Suit::Clubs),
    //         Card::new(Rank::King, Suit::Spades),
    //         Card::new(Rank::Jack, Suit::Spades),
    //     ];

    //     let holding = HoldingSlice::new(&cards[..2]);
    //     assert_eq!(holding.cards.len(), 2);
    // }
}
