use pkr::error::{Error, Result};
use pkr::prelude::*;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::process;

fn print_result(ranks: &[[[usize; 4]; 13]; 2], hand_rank: &HandRank, other: &HandRank) {
    println!("{:?} vs {:?} ", hand_rank, other);
    for (rank, card_array) in ranks[0].iter().rev().enumerate() {
        for (idx, suit) in card_array.iter().enumerate() {
            if *suit == 1 as usize {
                print!("{} ", Card::new(Rank::from(12 - rank), Suit::from(idx)));
            }
        }
    }
    print!(" vs ");
    for (rank, card_array) in ranks[1].iter().rev().enumerate() {
        for (idx, suit) in card_array.iter().enumerate() {
            if *suit == 1 as usize {
                print!("{} ", Card::new(Rank::from(12 - rank), Suit::from(idx)));
            }
        }
    }
    println!("");
}

fn combos(holdings: Vec<&[Card]>, community_cards: &[Card], deck: HashSet<Card>) {
    // Stats
    let mut num_combos = 0;
    let k = 5 - community_cards.len();

    let mut stats: HashMap<usize, usize> = HashMap::with_capacity(10);

    let h = Hand::new(holdings[0], community_cards);
    let v = Hand::new(holdings[1], community_cards);

    for combo in deck.iter().combinations(k) {
        // let (ranks, num_ranks, num_suits) = setup_arrays(&holdings, &community_cards, &combo);

        let he = h.rank(combo.as_slice());
        let vi = v.rank(combo.as_slice());

        // let hero = Hand::bare(&ranks[0], &num_ranks[0], &num_suits[0]);
        // let vilan = Hand::bare(&ranks[1], &num_ranks[1], &num_suits[1]);

        // if hero.hand_rank != he {
        //     println!("{:?} vs. {:?}", hero.hand_rank, he)
        // }
        // if vilan.hand_rank != vi {
        //     println!("{:?} vs. {:?}", vilan.hand_rank, vi)
        // }

        if he > vi {
            // print_result(&ranks, &hero.hand_rank, &vilan.hand_rank);
            if let Some(count) = stats.get_mut(&usize::from(&he)) {
                *count += 1;
            } else {
                stats.insert(usize::from(&he), 1);
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
    let offset = if cmd == "rnd" { 2 } else { 1 };

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

    if cmd == "test" {
        let deck = Deck::new();
        let mut count = 0;
        for combo in deck.cards.iter().combinations(2) {
            count += 1;
            println!("{:?}", combo);
        }
        println!("{:?}", count);
    }
    Ok(())
}
