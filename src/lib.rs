pub mod card;
pub mod deck;
pub mod error;
pub mod hand;
pub mod hand_rank;
pub mod prelude;

use std::collections::HashMap;

/// A trait to determine wheter Self beats, splits or looses against another
pub trait Beats<Rhs: ?Sized = Self> {
    fn beats(&self, other: &Rhs) -> bool;

    fn pairs(&self, other: &Rhs) -> bool;

    fn looses(&self, other: &Rhs) -> bool {
        !self.beats(other) && !self.pairs(other)
    }
}

pub fn setup_arrays(
    holdings: &Vec<&[card::Card]>,
    community_cards: &[card::Card],
    combo: &Vec<&card::Card>,
) -> ([[[usize; 4]; 13]; 2], [[usize; 13]; 2], [[usize; 4]; 2]) {
    // one array per player
    let mut ranks = [
        [
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
        ],
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ],
    ];

    // number of cards from Two -> Ace
    let mut num_ranks = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    // number of suites from Clubs to Diamonds
    let mut num_suits = [[0, 0, 0, 0], [0, 0, 0, 0]];

    // populating the arrays with the holdings
    for (i, holding) in holdings.iter().enumerate() {
        for card in holding.iter() {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[i][rank][suit] = 1;
            num_ranks[i][rank] += 1;
            num_suits[i][suit] += 1;
        }
    }

    // populating the arrays with the community cards
    for card in community_cards {
        let rank = card.rank as usize;
        let suit = card.suit as usize;
        for i in 0..holdings.len() {
            ranks[i][rank][suit] = 1;
            num_ranks[i][rank] += 1;
            num_suits[i][suit] += 1;
        }
    }

    // finally populating the array with the varying combos
    for card in combo {
        let rank = card.rank as usize;
        let suit = card.suit as usize;
        for i in 0..holdings.len() {
            ranks[i][rank][suit] = 1;
            num_ranks[i][rank] += 1;
            num_suits[i][suit] += 1;
        }
    }
    (ranks, num_ranks, num_suits)
}

pub fn print_rnd(stats: HashMap<usize, usize>, num: usize) {
    println!("evaluated {} random hands", num);
    println!("-> hero wins with:");
    let mut wins = 0;
    for i in 0..9 {
        if let Some((_rank, n)) = stats.get_key_value(&i) {
            wins += n;
            println!(
                "{:>11}: {:>6.2}% ({})",
                format!("{}", hand_rank::HandRank::from(i)),
                *n as f64 * 100.0 / num as f64,
                n
            );
        }
    }
}

pub fn print_combos(stats: HashMap<usize, usize>, num: usize, k: usize, len: usize) {
    println!("evaluated {} combinations for {}/{} cards", num, k, len);
    println!("-> hero wins with:");

    let mut wins = 0;
    for i in 0..9 {
        if let Some((_rank, n)) = stats.get_key_value(&i) {
            wins += n;
            println!(
                "{:>11}: {:>6.2}% ({})",
                format!("{}", hand_rank::HandRank::from(i)),
                *n as f64 * 100.0 / num as f64,
                n
            );
        }
    }
}
