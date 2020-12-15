use crate::error::{Error, Result};
use crate::Beats;
use std::fmt;

/// Suit
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

/// Rank
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Card
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn from(expr: &str) -> Result<Self> {
        if expr.len() != 2 {
            return Err(Error::ParseError);
        }

        let rank = match &expr[..1] {
            "A" => Rank::Ace,
            "K" => Rank::King,
            "Q" => Rank::Queen,
            "J" => Rank::Jack,
            "T" => Rank::Ten,
            "9" => Rank::Nine,
            "8" => Rank::Eight,
            "7" => Rank::Seven,
            "6" => Rank::Six,
            "5" => Rank::Five,
            "4" => Rank::Four,
            "3" => Rank::Three,
            "2" => Rank::Two,
            _ => return Err(Error::ParseError),
        };

        let suit = match &expr[1..2] {
            "c" => Suit::Clubs,
            "s" => Suit::Spades,
            "h" => Suit::Hearts,
            "d" => Suit::Diamonds,
            _ => return Err(Error::ParseError),
        };
        Ok(Card { rank, suit })
    }
}

impl From<usize> for Rank {
    fn from(n: usize) -> Self {
        match n {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Suit {
    fn from(n: usize) -> Self {
        match n {
            0 => Suit::Clubs,
            1 => Suit::Spades,
            2 => Suit::Hearts,
            3 => Suit::Diamonds,
            _ => unreachable!(),
        }
    }
}

/// Determine the rank equality for a Card despite its Suit
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Beats for Card {
    fn beats(&self, other: &Self) -> bool {
        self.rank > other.rank
    }

    fn pairs(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "{}", "\u{2663}"),
            Suit::Spades => write!(f, "{}", "\u{2660}"),
            Suit::Hearts => write!(f, "{}", "\u{2764}"),
            Suit::Diamonds => write!(f, "{}", "\u{2666}"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "{}", "A"),
            Rank::King => write!(f, "{}", "K"),
            Rank::Queen => write!(f, "{}", "Q"),
            Rank::Jack => write!(f, "{}", "J"),
            Rank::Ten => write!(f, "{}", "T"),
            Rank::Nine => write!(f, "{}", "9"),
            Rank::Eight => write!(f, "{}", "8"),
            Rank::Seven => write!(f, "{}", "7"),
            Rank::Six => write!(f, "{}", "6"),
            Rank::Five => write!(f, "{}", "5"),
            Rank::Four => write!(f, "{}", "4"),
            Rank::Three => write!(f, "{}", "3"),
            Rank::Two => write!(f, "{}", "2"),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suit_from() {
        assert_eq!(Suit::from(0), Suit::Clubs);
        assert_eq!(Suit::from(1), Suit::Spades);
        assert_eq!(Suit::from(2), Suit::Hearts);
        assert_eq!(Suit::from(3), Suit::Diamonds);
    }

    #[test]
    fn rank_from() {
        assert_eq!(Rank::from(0), Rank::Two);
        assert_eq!(Rank::from(1), Rank::Three);
        assert_eq!(Rank::from(2), Rank::Four);
        assert_eq!(Rank::from(3), Rank::Five);
        assert_eq!(Rank::from(4), Rank::Six);
        assert_eq!(Rank::from(5), Rank::Seven);
        assert_eq!(Rank::from(6), Rank::Eight);
        assert_eq!(Rank::from(7), Rank::Nine);
        assert_eq!(Rank::from(8), Rank::Ten);
        assert_eq!(Rank::from(9), Rank::Jack);
        assert_eq!(Rank::from(10), Rank::Queen);
        assert_eq!(Rank::from(11), Rank::King);
        assert_eq!(Rank::from(12), Rank::Ace);
    }

    #[test]
    fn from_expr() {
        // Ace
        assert_eq!(Card::from("Ac").unwrap(), Card::new(Rank::Ace, Suit::Clubs));
        assert_eq!(
            Card::from("As").unwrap(),
            Card::new(Rank::Ace, Suit::Spades)
        );
        assert_eq!(
            Card::from("Ah").unwrap(),
            Card::new(Rank::Ace, Suit::Hearts)
        );
        assert_eq!(
            Card::from("Ad").unwrap(),
            Card::new(Rank::Ace, Suit::Diamonds)
        );
        // King
        assert_eq!(
            Card::from("Kc").unwrap(),
            Card::new(Rank::King, Suit::Clubs)
        );
        assert_eq!(
            Card::from("Ks").unwrap(),
            Card::new(Rank::King, Suit::Spades)
        );
        assert_eq!(
            Card::from("Kh").unwrap(),
            Card::new(Rank::King, Suit::Hearts)
        );
        assert_eq!(
            Card::from("Kd").unwrap(),
            Card::new(Rank::King, Suit::Diamonds)
        );
        // Queen
        assert_eq!(
            Card::from("Qc").unwrap(),
            Card::new(Rank::Queen, Suit::Clubs)
        );
        assert_eq!(
            Card::from("Qs").unwrap(),
            Card::new(Rank::Queen, Suit::Spades)
        );
        assert_eq!(
            Card::from("Qh").unwrap(),
            Card::new(Rank::Queen, Suit::Hearts)
        );
        assert_eq!(
            Card::from("Qd").unwrap(),
            Card::new(Rank::Queen, Suit::Diamonds)
        );
        // Jack
        assert_eq!(
            Card::from("Jc").unwrap(),
            Card::new(Rank::Jack, Suit::Clubs)
        );
        assert_eq!(
            Card::from("Js").unwrap(),
            Card::new(Rank::Jack, Suit::Spades)
        );
        assert_eq!(
            Card::from("Jh").unwrap(),
            Card::new(Rank::Jack, Suit::Hearts)
        );
        assert_eq!(
            Card::from("Jd").unwrap(),
            Card::new(Rank::Jack, Suit::Diamonds)
        );
        // Ten
        assert_eq!(Card::from("Tc").unwrap(), Card::new(Rank::Ten, Suit::Clubs));
        assert_eq!(
            Card::from("Ts").unwrap(),
            Card::new(Rank::Ten, Suit::Spades)
        );
        assert_eq!(
            Card::from("Th").unwrap(),
            Card::new(Rank::Ten, Suit::Hearts)
        );
        assert_eq!(
            Card::from("Td").unwrap(),
            Card::new(Rank::Ten, Suit::Diamonds)
        );
        // Nine
        assert_eq!(
            Card::from("9c").unwrap(),
            Card::new(Rank::Nine, Suit::Clubs)
        );
        assert_eq!(
            Card::from("9s").unwrap(),
            Card::new(Rank::Nine, Suit::Spades)
        );
        assert_eq!(
            Card::from("9h").unwrap(),
            Card::new(Rank::Nine, Suit::Hearts)
        );
        assert_eq!(
            Card::from("9d").unwrap(),
            Card::new(Rank::Nine, Suit::Diamonds)
        );
        // Eight
        assert_eq!(
            Card::from("8c").unwrap(),
            Card::new(Rank::Eight, Suit::Clubs)
        );
        assert_eq!(
            Card::from("8s").unwrap(),
            Card::new(Rank::Eight, Suit::Spades)
        );
        assert_eq!(
            Card::from("8h").unwrap(),
            Card::new(Rank::Eight, Suit::Hearts)
        );
        assert_eq!(
            Card::from("8d").unwrap(),
            Card::new(Rank::Eight, Suit::Diamonds)
        );
        // Seven
        assert_eq!(
            Card::from("7c").unwrap(),
            Card::new(Rank::Seven, Suit::Clubs)
        );
        assert_eq!(
            Card::from("7s").unwrap(),
            Card::new(Rank::Seven, Suit::Spades)
        );
        assert_eq!(
            Card::from("7h").unwrap(),
            Card::new(Rank::Seven, Suit::Hearts)
        );
        assert_eq!(
            Card::from("7d").unwrap(),
            Card::new(Rank::Seven, Suit::Diamonds)
        );
        // Six
        assert_eq!(Card::from("6c").unwrap(), Card::new(Rank::Six, Suit::Clubs));
        assert_eq!(
            Card::from("6s").unwrap(),
            Card::new(Rank::Six, Suit::Spades)
        );
        assert_eq!(
            Card::from("6h").unwrap(),
            Card::new(Rank::Six, Suit::Hearts)
        );
        assert_eq!(
            Card::from("6d").unwrap(),
            Card::new(Rank::Six, Suit::Diamonds)
        );
        // Five
        assert_eq!(
            Card::from("5c").unwrap(),
            Card::new(Rank::Five, Suit::Clubs)
        );
        assert_eq!(
            Card::from("5s").unwrap(),
            Card::new(Rank::Five, Suit::Spades)
        );
        assert_eq!(
            Card::from("5h").unwrap(),
            Card::new(Rank::Five, Suit::Hearts)
        );
        assert_eq!(
            Card::from("5d").unwrap(),
            Card::new(Rank::Five, Suit::Diamonds)
        );
        // Four
        assert_eq!(
            Card::from("4c").unwrap(),
            Card::new(Rank::Four, Suit::Clubs)
        );
        assert_eq!(
            Card::from("4s").unwrap(),
            Card::new(Rank::Four, Suit::Spades)
        );
        assert_eq!(
            Card::from("4h").unwrap(),
            Card::new(Rank::Four, Suit::Hearts)
        );
        assert_eq!(
            Card::from("4d").unwrap(),
            Card::new(Rank::Four, Suit::Diamonds)
        );
        // Three
        assert_eq!(
            Card::from("3c").unwrap(),
            Card::new(Rank::Three, Suit::Clubs)
        );
        assert_eq!(
            Card::from("3s").unwrap(),
            Card::new(Rank::Three, Suit::Spades)
        );
        assert_eq!(
            Card::from("3h").unwrap(),
            Card::new(Rank::Three, Suit::Hearts)
        );
        assert_eq!(
            Card::from("3d").unwrap(),
            Card::new(Rank::Three, Suit::Diamonds)
        );
        // Two
        assert_eq!(Card::from("2c").unwrap(), Card::new(Rank::Two, Suit::Clubs));
        assert_eq!(
            Card::from("2s").unwrap(),
            Card::new(Rank::Two, Suit::Spades)
        );
        assert_eq!(
            Card::from("2h").unwrap(),
            Card::new(Rank::Two, Suit::Hearts)
        );
        assert_eq!(
            Card::from("2d").unwrap(),
            Card::new(Rank::Two, Suit::Diamonds)
        );

        // Errors
        assert!(Card::from("As").is_ok()); // Card is ok
        assert!(Card::from("As+").is_err()); // Card cannot be a hand range
        assert!(Card::from("23s").is_err()); // Card cannot be suited connectors
        assert!(Card::from("1s").is_err()); // Card cannot be a hand range
    }

    #[test]
    fn ranks() {
        // Ranks
        assert_eq!(Rank::Ace > Rank::King, true);
        assert_eq!(Rank::King > Rank::Queen, true);
        assert_eq!(Rank::Queen > Rank::Jack, true);
        assert_eq!(Rank::Jack > Rank::Ten, true);
        assert_eq!(Rank::Ten > Rank::Nine, true);
        assert_eq!(Rank::Nine > Rank::Eight, true);
        assert_eq!(Rank::Eight > Rank::Seven, true);
        assert_eq!(Rank::Seven > Rank::Six, true);
        assert_eq!(Rank::Six > Rank::Five, true);
        assert_eq!(Rank::Five > Rank::Four, true);
        assert_eq!(Rank::Four > Rank::Three, true);
        assert_eq!(Rank::Three > Rank::Two, true);
    }

    #[test]
    fn beats() {
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).beats(&Card::new(Rank::Ace, Suit::Clubs)),
            false
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).beats(&Card::new(Rank::Ace, Suit::Spades)),
            false
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).beats(&Card::new(Rank::Ace, Suit::Hearts)),
            false
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).beats(&Card::new(Rank::Ace, Suit::Diamonds)),
            false
        );

        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).beats(&Card::new(Rank::King, Suit::Diamonds)),
            true
        );
    }

    #[test]
    fn pairs() {
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).pairs(&Card::new(Rank::Ace, Suit::Spades)),
            true
        );

        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).pairs(&Card::new(Rank::King, Suit::Spades)),
            false
        );
    }

    #[test]
    fn looses() {
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).looses(&Card::new(Rank::Ace, Suit::Clubs)),
            false
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).looses(&Card::new(Rank::Ace, Suit::Spades)),
            false
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).looses(&Card::new(Rank::Ace, Suit::Hearts)),
            false
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).looses(&Card::new(Rank::Ace, Suit::Diamonds)),
            false
        );

        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs).looses(&Card::new(Rank::King, Suit::Diamonds)),
            false
        );
    }

    #[test]
    fn partial_eq() {
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs) == Card::new(Rank::Ace, Suit::Clubs),
            true
        );
        assert_eq!(
            Card::new(Rank::King, Suit::Spades) == Card::new(Rank::King, Suit::Spades),
            true
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Hearts) == Card::new(Rank::Ace, Suit::Hearts),
            true
        );
    }

    #[test]
    fn partial_ne() {
        assert_ne!(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Diamonds)
        );
        assert_ne!(
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Diamonds)
        );
    }

    #[test]
    fn partial_ord() {
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs) > Card::new(Rank::King, Suit::Clubs),
            true
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs) > Card::new(Rank::King, Suit::Spades),
            true
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs) > Card::new(Rank::King, Suit::Hearts),
            true
        );
        assert_eq!(
            Card::new(Rank::Ace, Suit::Clubs) > Card::new(Rank::King, Suit::Diamonds),
            true
        );

        assert_eq!(
            Card::new(Rank::King, Suit::Diamonds) < Card::new(Rank::Ace, Suit::Clubs),
            true
        );
    }

    #[test]
    fn sort() {
        let mut slice = [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Diamonds),
        ];
        slice.sort();

        assert_eq!(
            slice,
            [
                Card::new(Rank::King, Suit::Diamonds),
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Spades),
            ]
        );
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Card>(), 2);
        assert_eq!(std::mem::size_of::<Rank>(), 1);
        assert_eq!(std::mem::size_of::<Suit>(), 1);
    }
}
