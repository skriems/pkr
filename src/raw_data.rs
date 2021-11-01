use std::{iter::Chain, slice::Iter};

use crate::card::*;

type Deck = [[usize; 4]; 13];
type NumRanks = [usize; 13];
type NumSuits = [usize; 4];

#[derive(Debug)]
pub struct RawData {
    pub ranks: Deck,
    pub num_ranks: NumRanks,
    pub num_suits: NumSuits,
}

impl RawData {
    pub fn from_chain(chain: Chain<Iter<Card>, Iter<Card>>) -> RawData {
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

        for card in chain.into_iter() {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        RawData {
            ranks,
            num_ranks,
            num_suits,
        }
    }

    pub fn new(cards: &Vec<&Card>) -> RawData {
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

        RawData {
            ranks,
            num_ranks,
            num_suits,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         let card1 = Card::new(Rank::Ace, Suit::Spades);
//         let card2 = Card::new(Rank::Ace, Suit::Clubs);
//         let cards = vec![&card1, &card2];
//         let sut = RawData::new(cards);

//         assert_eq!(sut.ranks[Rank::Ace as usize][Suit::Spades as usize], 1);
//         assert_eq!(sut.ranks[Rank::Ace as usize][Suit::Clubs as usize], 1);
//     }
// }
