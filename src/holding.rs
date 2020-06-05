use crate::card::*;
use crate::error::{Error, Result};

/// A Players Holding Cards
#[derive(Debug, PartialOrd)]
pub struct Holding {
    first: Card,
    second: Card,
}

impl Holding {
    pub fn new(first: Card, second: Card) -> Result<Self> {
        if first.rank == second.rank && first.suit == second.suit {
            return Err(Error::ParseError);
        }
        Ok(Holding { first, second })
    }

    pub fn from(expr: &str) -> Result<Self> {
        if expr.len() != 4 {
            return Err(Error::ParseError);
        }

        let first = Card::from(&expr[..2])?;
        let second = Card::from(&expr[2..4])?;
        Holding::new(first, second)
    }
}

impl PartialEq for Holding {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
            || self.first == other.second && self.second == other.first
            || self.second == other.first && self.first == other.second
            || self.second == other.second && self.first == other.first
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // same Rank, different Suit
        assert!(Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades)
        )
        .is_ok());

        // different Rank, suited
        assert!(Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs)
        )
        .is_ok());

        // same Rank, suited
        assert!(Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs)
        )
        .is_err());
    }

    #[test]
    fn from_expr() {
        // different Rank, suited
        assert!(Holding::from("AsKs").is_ok());
        // same cards
        assert!(Holding::from("AsAs").is_err());
        // same Rank, different Suit
        assert!(Holding::from("AsAd").is_ok());
        // different Rank, different Suit
        assert!(Holding::from("AsKc").is_ok());
    }

    #[test]
    fn partial_eq() {
        // Aces
        let first = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        )
        .unwrap();

        assert_eq!(first, second);

        // AK's
        let first = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
        )
        .unwrap();

        assert_eq!(first, second);

        // KA vs AK
        let first = Holding::new(
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();

        assert_eq!(first, second);

        // AK vs KA
        let first = Holding::from("AsKs").unwrap();
        let second = Holding::from("KhAd").unwrap();
        assert_eq!(first, second);
        // same cards
        let first = Holding::from("AsKs").unwrap();
        let second = Holding::from("AsKs").unwrap();
        assert_eq!(first, second);
        // same cards
        let first = Holding::from("AsKs").unwrap();
        let second = Holding::from("KsAs").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn partial_eq_ne() {
        // AK vs AQ
        let first = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
        )
        .unwrap();

        assert!(first != second);
    }

    #[test]
    fn partial_ord() {
        // Aces vs Kings
        let first = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();

        assert!(first > second);

        // AK vs AQ
        let first = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
        )
        .unwrap();

        assert!(first > second);

        // AQ vs AK
        let first = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
        )
        .unwrap();
        let second = Holding::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();

        assert!(first < second);
    }
}
