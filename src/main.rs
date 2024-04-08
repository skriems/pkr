use pkr::error::Result;
use pkr::prelude::*;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::process;

fn combos(hero: Vec<Card>, vilan: Vec<Card>, community_cards: Vec<Card>, deck: HashSet<Card>) {
    // Stats
    let mut stats: HashMap<usize, usize> = HashMap::with_capacity(10);
    let mut num_combos = 0;
    let k = 5 - community_cards.len();

    let raw_hero = RawData::from_chain(hero.iter().chain(community_cards.iter()));
    let raw_vilan = RawData::from_chain(vilan.iter().chain(community_cards.iter()));

    for combo in deck.iter().combinations(k) {
        let h = Hand::new(&raw_hero, &combo);
        let v = Hand::new(&raw_vilan, &combo);

        if h.rank > v.rank {
            // print_result(&ranks, &hero.hand_rank, &vilan.hand_rank);
            if let Some(count) = stats.get_mut(&usize::from(&h.rank)) {
                *count += 1;
            } else {
                stats.insert(usize::from(&h.rank), 1);
            }
        } else if h.rank == v.rank {
            // println!("{:?} vs {:?}", h.rank, v.rank)
        }
        num_combos += 1;
    }
    print_combos(stats, num_combos, k, deck.len());
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
    let offset = if cmd == "rnd" { 2 } else { 1 };

    let (hero, vilan, community_cards, deck) = get_cards(&args[offset..])?;

    // println!("{:?} vs. {:?} {:?}", hero, vilan, community_cards);
    if cmd == "eval" {
        combos(hero, vilan, community_cards, deck);
    }

    // if cmd == "test" {
    //     let deck = Deck::new();
    //     let mut count = 0;
    //     for combo in deck.cards.iter().combinations(2) {
    //         count += 1;
    //         println!("{:?}", combo);
    //     }
    //     println!("{:?}", count);
    // }
    Ok(())
}
