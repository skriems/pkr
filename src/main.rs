use pkr::error::{Error, Result};
use pkr::prelude::*;

use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::env;
use std::process;

struct StringChunks<'a> {
    slice: &'a str,
    step: usize,
}

impl<'a> StringChunks<'a> {
    fn new(slice: &'a str, step: usize) -> StringChunks<'a> {
        StringChunks { slice, step }
    }
}

impl<'a> Iterator for StringChunks<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.slice.is_empty() {
            return None;
        }
        let (ret, rest) = self.slice.split_at(self.step);
        self.slice = rest;
        Some(ret)
    }
}

// fn rnd(holdings: Vec<&[Card]>, community_cards: &[Card], deck: HashSet<Card>, iterations: usize) {
//     let mut remaining: Vec<&Card> = deck.iter().collect();
//     let mut rng = ThreadRng::default();

//     // Stats
//     let mut num_combos = 0;
//     let k = 5 - community_cards.len();

//     let mut stats: HashMap<usize, usize> = HashMap::with_capacity(10);

//     while num_combos < iterations {
//         remaining.shuffle(&mut rng);
//         let combo = &remaining[..k];
//         let (ranks, num_ranks, num_suits) = setup_arrays(&holdings, &community_cards, &combo);
//         let hero = HandResult::bare(&ranks[0], &num_ranks[0], &num_suits[0]);
//         let vilan = HandResult::bare(&ranks[1], &num_ranks[1], &num_suits[1]);

//         if hero > vilan {
//             if let Some(count) = stats.get_mut(&usize::from(&hero.hand_rank)) {
//                 *count += 1;
//             } else {
//                 stats.insert(usize::from(&hero.hand_rank), 1);
//             }
//         }
//         num_combos += 1;
//     }
//     print_rnd(stats, num_combos);
// }

fn combos(holdings: Vec<&[Card]>, community_cards: &[Card], deck: HashSet<Card>) {
    // Stats
    let mut num_combos = 0;
    let k = 5 - community_cards.len();

    let mut stats: HashMap<usize, usize> = HashMap::with_capacity(10);

    for combo in deck.iter().combinations(k) {
        let (ranks, num_ranks, num_suits) = setup_arrays(&holdings, &community_cards, &combo);
        let hero = HandResult::bare(&ranks[0], &num_ranks[0], &num_suits[0]);
        let vilan = HandResult::bare(&ranks[1], &num_ranks[1], &num_suits[1]);

        if hero.hand_rank == HandRank::RoyalFlush {
            print!("{:?}: ", &hero.hand_rank);
            for (rank, card_array) in ranks[0].iter().enumerate() {
                for suit in card_array {
                    if *suit == 1 as usize {
                        print!("{} ", Card::new(Rank::from(rank), Suit::from(*suit)));
                    }
                }
            }
            println!("");
        }

        if hero > vilan {
            if let Some(count) = stats.get_mut(&usize::from(&hero.hand_rank)) {
                *count += 1;
            } else {
                stats.insert(usize::from(&hero.hand_rank), 1);
            }
        }
        num_combos += 1;
    }
    print_combos(stats, num_combos, k, deck.len());
}

fn get_cards(args: &[String]) -> Result<(Vec<Card>, HashSet<Card>)> {
    let mut deck: HashSet<Card> = HashSet::new();
    deck.insert(Card::new(Rank::Ace, Suit::Clubs));
    deck.insert(Card::new(Rank::King, Suit::Clubs));
    deck.insert(Card::new(Rank::Queen, Suit::Clubs));
    deck.insert(Card::new(Rank::Jack, Suit::Clubs));
    deck.insert(Card::new(Rank::Ten, Suit::Clubs));
    deck.insert(Card::new(Rank::Nine, Suit::Clubs));
    deck.insert(Card::new(Rank::Eight, Suit::Clubs));
    deck.insert(Card::new(Rank::Seven, Suit::Clubs));
    deck.insert(Card::new(Rank::Six, Suit::Clubs));
    deck.insert(Card::new(Rank::Five, Suit::Clubs));
    deck.insert(Card::new(Rank::Four, Suit::Clubs));
    deck.insert(Card::new(Rank::Three, Suit::Clubs));
    deck.insert(Card::new(Rank::Two, Suit::Clubs));
    deck.insert(Card::new(Rank::Ace, Suit::Spades));
    deck.insert(Card::new(Rank::King, Suit::Spades));
    deck.insert(Card::new(Rank::Queen, Suit::Spades));
    deck.insert(Card::new(Rank::Jack, Suit::Spades));
    deck.insert(Card::new(Rank::Ten, Suit::Spades));
    deck.insert(Card::new(Rank::Nine, Suit::Spades));
    deck.insert(Card::new(Rank::Eight, Suit::Spades));
    deck.insert(Card::new(Rank::Seven, Suit::Spades));
    deck.insert(Card::new(Rank::Six, Suit::Spades));
    deck.insert(Card::new(Rank::Five, Suit::Spades));
    deck.insert(Card::new(Rank::Four, Suit::Spades));
    deck.insert(Card::new(Rank::Three, Suit::Spades));
    deck.insert(Card::new(Rank::Two, Suit::Spades));
    deck.insert(Card::new(Rank::Ace, Suit::Hearts));
    deck.insert(Card::new(Rank::King, Suit::Hearts));
    deck.insert(Card::new(Rank::Queen, Suit::Hearts));
    deck.insert(Card::new(Rank::Jack, Suit::Hearts));
    deck.insert(Card::new(Rank::Ten, Suit::Hearts));
    deck.insert(Card::new(Rank::Nine, Suit::Hearts));
    deck.insert(Card::new(Rank::Eight, Suit::Hearts));
    deck.insert(Card::new(Rank::Seven, Suit::Hearts));
    deck.insert(Card::new(Rank::Six, Suit::Hearts));
    deck.insert(Card::new(Rank::Five, Suit::Hearts));
    deck.insert(Card::new(Rank::Four, Suit::Hearts));
    deck.insert(Card::new(Rank::Three, Suit::Hearts));
    deck.insert(Card::new(Rank::Two, Suit::Hearts));
    deck.insert(Card::new(Rank::Ace, Suit::Diamonds));
    deck.insert(Card::new(Rank::King, Suit::Diamonds));
    deck.insert(Card::new(Rank::Queen, Suit::Diamonds));
    deck.insert(Card::new(Rank::Jack, Suit::Diamonds));
    deck.insert(Card::new(Rank::Ten, Suit::Diamonds));
    deck.insert(Card::new(Rank::Nine, Suit::Diamonds));
    deck.insert(Card::new(Rank::Eight, Suit::Diamonds));
    deck.insert(Card::new(Rank::Seven, Suit::Diamonds));
    deck.insert(Card::new(Rank::Six, Suit::Diamonds));
    deck.insert(Card::new(Rank::Five, Suit::Diamonds));
    deck.insert(Card::new(Rank::Four, Suit::Diamonds));
    deck.insert(Card::new(Rank::Three, Suit::Diamonds));
    deck.insert(Card::new(Rank::Two, Suit::Diamonds));

    let mut dealt: Vec<Card> = Vec::with_capacity(7); // 2 x holdings + flop + turn + river

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        for card_string in StringChunks::new(&arg, 2) {
            if let Ok(card) = Card::from(card_string) {
                dealt.push(deck.take(&card).ok_or(Error::DuplicateCard)?);
            }
        }
    }
    Ok((dealt, deck))
}

fn print_usage() {
    println!("usage: <cmd> [NUM_ITERATIONS] <Holding> <Holding> [COMMUNITY_CARDS..]");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_usage();
        process::exit(1);
    }

    let cmd = &args[1];
    let offset = if cmd == "eval" { 1 } else { 2 };

    let (dealt, deck) = get_cards(&args[offset..])?;

    let mut holdings: Vec<&[Card]> = Vec::with_capacity(dealt.len());
    for i in 0..2 {
        let start = i * 2;
        holdings.push(&dealt[start..start + 2]);
    }

    let community_cards = &dealt[2 * 2..];

    if cmd == "eval" {
        combos(holdings, community_cards, deck);
    }

    // } else if cmd == "rnd" {
    //     if let Ok(iterations) = &args[2].parse::<usize>() {
    //         rnd(holdings, community_cards, deck, *iterations, true);
    //     } else {
    //         print_usage();
    //     }
    // }

    // let deck = Deck::new();
    // for combo in deck.cards.iter().combinations(2) {
    //     println!("{:?}", combo)
    // }

    Ok(())
}
