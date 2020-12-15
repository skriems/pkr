use crate::card::*;
use std::fmt;

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
            5 => HandRank::Flush(42),
            6 => HandRank::FullHouse(Rank::Ace, Rank::King),
            7 => HandRank::Quads(Rank::Ace),
            8 => HandRank::StraightFlush(Rank::Ace),
            9 => HandRank::RoyalFlush,
            _ => unreachable!(),
        }
    }
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
    fn mem() {
        assert_eq!(std::mem::size_of::<HandRank>(), 16);
    }
}
