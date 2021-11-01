pub mod card;
pub mod cli;
pub mod deck;
pub mod error;
pub mod hand;
pub mod hand_rank;
pub mod prelude;
pub mod raw_data;

use std::collections::HashMap;

/// A trait to determine wheter Self beats, splits or looses against another
pub trait Beats<Rhs: ?Sized = Self> {
    fn beats(&self, other: &Rhs) -> bool;

    fn pairs(&self, other: &Rhs) -> bool;

    fn looses(&self, other: &Rhs) -> bool {
        !self.beats(other) && !self.pairs(other)
    }
}

pub fn print_combos(stats: HashMap<usize, usize>, num: usize, k: usize, len: usize) {
    println!("evaluated {} combinations for {}/{} cards", num, k, len);
    println!("-> hero wins with:");

    for i in 0..10 {
        let mut wins = 0;
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

pub fn print_result(
    ranks: &[[[usize; 4]; 13]; 2],
    hand_rank: &hand_rank::HandRank,
    other: &hand_rank::HandRank,
) {
    println!("{:?} vs {:?} ", hand_rank, other);
    for (rank, card_array) in ranks[0].iter().rev().enumerate() {
        for (idx, suit) in card_array.iter().enumerate() {
            if *suit == 1 as usize {
                print!(
                    "{} ",
                    card::Card::new(card::Rank::from(12 - rank), card::Suit::from(idx))
                );
            }
        }
    }
    print!(" vs ");
    for (rank, card_array) in ranks[1].iter().rev().enumerate() {
        for (idx, suit) in card_array.iter().enumerate() {
            if *suit == 1 as usize {
                print!(
                    "{} ",
                    card::Card::new(card::Rank::from(12 - rank), card::Suit::from(idx))
                );
            }
        }
    }
    println!("");
}
