use crate::card::*;
use crate::error::{Error, Result};
use crate::Beats;
use std::fmt;

/// A Players Holding Cards
#[derive(Debug, PartialOrd)]
pub struct Holding<'a> {
    pub cards: &'a [Card],
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

    pub fn is_suited(&self) -> bool {
        self.cards[0].suit == self.cards[1].suit
    }

    /// two cards are connected when the product of the difference of their respective
    /// enum discriminants is 1:
    ///
    /// connected:
    ///     Rank::Ace(12) - Rank::King(11) == 1
    ///     Rank::King(11) - Rank::Ace(12) == -1    <- hence product is needed
    ///
    /// not connected:
    ///     Rank::Ace(12) - Rank::Queen(10) == 2
    ///     Rank::Queen(10) - Rank::Ace(12) == -2
    pub fn is_connected(&self) -> bool {
        let res = self.cards[0].rank as u32 - self.cards[1].rank as u32;
        res * res == 1
    }
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

impl<'a> fmt::Display for Holding<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.high_card(), self.low_card())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // same Rank, different Suit
        let cards = [
            Card::from("Ac").unwrap(),
            Card::from("As").unwrap(),
        ];
        assert!(Holding::new(&cards).is_ok());
        assert_eq!(Holding::new(&cards).unwrap(), Holding { cards: &cards });

        // different Rank, suited
        let cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        assert!(Holding::new(&cards).is_ok());
        assert_eq!(Holding::new(&cards).unwrap(), Holding { cards: &cards });

        // same Rank, suited
        let cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        assert!(Holding::new(&cards).is_err());
    }

    #[test]
    fn high_card() {
        // AK -> &A
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let holding = Holding::new(&first_cards).unwrap();
        assert_eq!(holding.high_card(), &Card::from("Ac").unwrap());
    }

    #[test]
    fn low_card() {
        // AK -> &K
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let holding = Holding::new(&first_cards).unwrap();
        assert_eq!(holding.low_card(), &Card::from("Kc").unwrap());
    }

    #[test]
    fn beats() {
        // two overcards: AK vs QJ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("Qs").unwrap(),
            Card::from("Js").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // two overcards: KA vs QJ
        let first_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("Qs").unwrap(),
            Card::from("Js").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // two overcards: AK vs JQ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("Js").unwrap(),
            Card::from("Qs").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: AK vs AQ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Qs").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: KA vs AQ
        let first_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Qs").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: AK vs QA
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("Qs").unwrap(),
            Card::from("As").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // better kicker: KA vs QA
        let first_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("Qs").unwrap(),
            Card::from("As").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // AK vs AK
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Kd").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // KA vs AK
        let first_cards = [
            Card::from("Ks").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Kd").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // AK vs KA
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let second_cards = [
            Card::from("Kd").unwrap(),
            Card::from("As").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // KA vs KA
        let first_cards = [
            Card::from("Ks").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("Kd").unwrap(),
            Card::from("As").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // AJ vs KQ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Js").unwrap(),
        ];
        let second_cards = [
            Card::from("Kd").unwrap(),
            Card::from("Qs").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // J7 vs T8
        let first_cards = [
            Card::from("Js").unwrap(),
            Card::from("7c").unwrap(),
        ];
        let second_cards = [
            Card::from("Td").unwrap(),
            Card::from("8s").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // 55 vs AK
        let first_cards = [
            Card::from("5s").unwrap(),
            Card::from("5c").unwrap(),
        ];
        let second_cards = [
            Card::from("Ad").unwrap(),
            Card::from("Ks").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // 55 vs 44
        let first_cards = [
            Card::from("5s").unwrap(),
            Card::from("5c").unwrap(),
        ];
        let second_cards = [
            Card::from("4d").unwrap(),
            Card::from("4s").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), true);

        // 55 vs 55
        let first_cards = [
            Card::from("5s").unwrap(),
            Card::from("5c").unwrap(),
        ];
        let second_cards = [
            Card::from("5d").unwrap(),
            Card::from("5s").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);

        // AK vs 55
        let first_cards = [
            Card::from("As").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("5d").unwrap(),
            Card::from("5s").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.beats(&other), false);
    }

    #[test]
    fn pairs() {
        // AK vs AK
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Ks").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // KA vs AK
        let first_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Ks").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // AK vs KA
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("Ks").unwrap(),
            Card::from("As").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // KA vs KA
        let first_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let second_cards = [
            Card::from("Ks").unwrap(),
            Card::from("As").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // AK vs AQ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Qs").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), false);

        // 87 vs 87
        let first_cards = [
            Card::from("8c").unwrap(),
            Card::from("7c").unwrap(),
        ];
        let second_cards = [
            Card::from("8s").unwrap(),
            Card::from("7s").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), true);

        // 55 vs 55
        let first_cards = [
            Card::from("5s").unwrap(),
            Card::from("5c").unwrap(),
        ];
        let second_cards = [
            Card::from("4d").unwrap(),
            Card::from("4s").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.pairs(&other), false);
    }

    #[test]
    fn looses() {
        // 44 vs 55
        let first_cards = [
            Card::from("4d").unwrap(),
            Card::from("4s").unwrap(),
        ];
        let second_cards = [
            Card::from("5s").unwrap(),
            Card::from("5c").unwrap(),
        ];

        let holding = Holding::new(&first_cards).unwrap();
        let other = Holding::new(&second_cards).unwrap();
        assert_eq!(holding.looses(&other), true);
    }

    #[test]
    fn partial_eq() {
        // Aces
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("As").unwrap(),
        ];
        let second_cards = [
            Card::from("As").unwrap(),
            Card::from("Ac").unwrap(),
        ];

        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert_eq!(first, second);

        // AK's
        let first_cards = [
            Card::from("Ah").unwrap(),
            Card::from("Kd").unwrap(),
        ];
        let second_cards = [
            Card::from("Kd").unwrap(),
            Card::from("Ah").unwrap(),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn partial_ne() {
        // AK vs AQ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let second_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Qd").unwrap(),
        ];

        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn partial_ord() {
        // Aces vs Kings
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("As").unwrap(),
        ];
        let second_cards = [
            Card::from("Kc").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert!(first > second);

        // AK vs AQ
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let second_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Qs").unwrap(),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert!(first > second);

        // AQ vs AK
        let first_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Qs").unwrap(),
        ];
        let second_cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let first = Holding::new(&first_cards).unwrap();
        let second = Holding::new(&second_cards).unwrap();

        assert!(first < second);
    }

    #[test]
    fn is_suited() {
        // AK
        let cards = [
            Card::from("As").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let holding = Holding::new(&cards).unwrap();

        assert_eq!(holding.is_suited(), true);

        // AK
        let cards = [
            Card::from("As").unwrap(),
            Card::from("Kc").unwrap(),
        ];
        let holding = Holding::new(&cards).unwrap();

        assert_eq!(holding.is_suited(), false);
    }

    #[test]
    fn is_connected() {
        // AK
        let cards = [
            Card::from("Ac").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let holding = Holding::new(&cards).unwrap();

        assert_eq!(holding.is_connected(), true);

        // AQ
        let cards = [
            Card::from("Ac").unwrap(),
            Card::from("Qs").unwrap(),
        ];

        let holding = Holding::new(&cards).unwrap();
        assert_eq!(holding.is_connected(), false);
    }

    #[test]
    fn is_pocket_pair() {
        // AA
        let cards = [
            Card::from("As").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let holding = Holding::new(&cards).unwrap();

        assert_eq!(holding.is_pocket_pair(), true);

        // AK
        let cards = [
            Card::from("As").unwrap(),
            Card::from("Ks").unwrap(),
        ];
        let holding = Holding::new(&cards).unwrap();

        assert_eq!(holding.is_pocket_pair(), false);
    }

    #[test]
    fn contains() {
        // AA
        let cards = [
            Card::from("As").unwrap(),
            Card::from("Ac").unwrap(),
        ];
        let card = Card::from("Ad").unwrap();

        let holding = Holding::new(&cards).unwrap();
        assert_eq!(holding.cards.contains(&card), true);

        // 66
        let cards = [
            Card::from("6s").unwrap(),
            Card::from("6c").unwrap(),
        ];
        let card = Card::from("6d").unwrap();

        let holding = Holding::new(&cards).unwrap();
        assert_eq!(holding.cards.contains(&card), true);
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Holding>(), 16);
    }
}
