use crate::card::*;
use crate::error::{Error, Result};

/// Hand
#[derive(Debug, PartialOrd)]
pub struct Hand {
    first: Card,
    second: Card,
}

impl Hand {
    pub fn new(first: Card, second: Card) -> Result<Self> {
        if first.rank == second.rank && first.suit == second.suit {
            return Err(Error::ParseError);
        }
        Ok(Hand { first, second })
    }

    pub fn from(expr: &str) -> Result<Self> {
        if expr.len() != 4 {
            return Err(Error::ParseError);
        }

        let first = Card::from(&expr[..2])?;
        let second = Card::from(&expr[2..4])?;
        Hand::new(first, second)
    }
}

impl PartialEq for Hand {
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
        assert!(Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades)
        )
        .is_ok());

        // different Rank, suited
        assert!(Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs)
        )
        .is_ok());

        // same Rank, suited
        assert!(Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs)
        )
        .is_err());
    }

    #[test]
    fn from_expr() {
        // different Rank, suited
        assert!(Hand::from("AsKs").is_ok());
        // same cards
        assert!(Hand::from("AsAs").is_err());
        // same Rank, different Suit
        assert!(Hand::from("AsAd").is_ok());
        // different Rank, different Suit
        assert!(Hand::from("AsKc").is_ok());
    }

    #[test]
    fn partial_eq() {
        // Aces
        let first = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        )
        .unwrap();

        assert_eq!(first, second);

        // AK's
        let first = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
        )
        .unwrap();

        assert_eq!(first, second);

        // KA vs AK
        let first = Hand::new(
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();

        assert_eq!(first, second);

        // AK vs KA
        let first = Hand::from("AsKs").unwrap();
        let second = Hand::from("KhAd").unwrap();
        assert_eq!(first, second);
        // same cards
        let first = Hand::from("AsKs").unwrap();
        let second = Hand::from("AsKs").unwrap();
        assert_eq!(first, second);
        // same cards
        let first = Hand::from("AsKs").unwrap();
        let second = Hand::from("KsAs").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn partial_eq_ne() {
        // AK vs AQ
        let first = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
        )
        .unwrap();

        assert!(first != second);
    }

    #[test]
    fn partial_ord() {
        // Aces vs Kings
        let first = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();

        assert!(first > second);

        // AK vs AQ
        let first = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
        )
        .unwrap();

        assert!(first > second);

        // AQ vs AK
        let first = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
        )
        .unwrap();
        let second = Hand::new(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        )
        .unwrap();

        assert!(first < second);
    }
}
