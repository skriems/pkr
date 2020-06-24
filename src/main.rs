use pkr::error::{Error, Result};
use pkr::prelude::*;

use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::env;
use std::process;

fn print_result(
    winner: &HandResult,
    winner_cards: &&[Card],
    looser: &HandResult,
    looser_cards: &&[Card],
) {
    println!(
        "[{}{}] vs. [{}{}] | \t{:?} vs. {:?}",
        winner_cards[0],
        winner_cards[1],
        looser_cards[0],
        looser_cards[1],
        winner.hand_rank,
        looser.hand_rank,
    );
}

fn rnd(
    holdings: Vec<&[Card]>,
    community_cards: &[Card],
    deck: HashSet<Card>,
    iterations: usize,
    benchmark: bool,
) {
    let mut remaining: Vec<&Card> = deck.iter().collect();
    let mut rng = ThreadRng::default();

    // Stats
    let mut num_combos = 0;
    let k = 5 - community_cards.len();

    let mut stats: HashMap<usize, usize> = HashMap::with_capacity(10);

    while num_combos < iterations {
        remaining.shuffle(&mut rng);
        let combo = &remaining[..k];

        let mut ranks = [
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

        let mut num_ranks = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut num_suits = [[0, 0, 0, 0], [0, 0, 0, 0]];

        for (i, holding) in holdings.iter().enumerate() {
            for card in holding.iter() {
                let rank = card.rank as usize;
                let suit = card.suit as usize;
                ranks[i][rank][suit] = 1;
                num_ranks[i][rank] += 1;
                num_suits[i][suit] += 1;
            }
        }

        for card in community_cards {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            for i in 0..holdings.len() {
                ranks[i][rank][suit] = 1;
                num_ranks[i][rank] += 1;
                num_suits[i][suit] += 1;
            }
        }

        for card in combo {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            for i in 0..holdings.len() {
                ranks[i][rank][suit] = 1;
                num_ranks[i][rank] += 1;
                num_suits[i][suit] += 1;
            }
        }

        let hero = HandResult::bare(&ranks[0], &num_ranks[0], &num_suits[0]);
        let vilan = HandResult::bare(&ranks[1], &num_ranks[1], &num_suits[1]);

        if hero > vilan {
            if let Some(count) = stats.get_mut(&usize::from(&hero.hand_rank)) {
                *count += 1;
            } else {
                stats.insert(usize::from(&hero.hand_rank), 1);
            }
        }
        num_combos += 1;
    }
    println!("-> evaluated {} random hands", num_combos);
    println!("{:#?}", stats);
}

fn combos(holdings: Vec<&[Card]>, community_cards: &[Card], deck: HashSet<Card>, benchmark: bool) {
    // Stats
    let mut num_combos = 0;
    let k = 5 - community_cards.len();

    let mut stats: HashMap<usize, usize> = HashMap::with_capacity(10);

    for combo in deck.iter().combinations(k) {
        let mut ranks = [
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

        let mut num_ranks = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let mut num_suits = [[0, 0, 0, 0], [0, 0, 0, 0]];

        for (i, holding) in holdings.iter().enumerate() {
            for card in holding.iter() {
                let rank = card.rank as usize;
                let suit = card.suit as usize;
                ranks[i][rank][suit] = 1;
                num_ranks[i][rank] += 1;
                num_suits[i][suit] += 1;
            }
        }

        for card in community_cards {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            for i in 0..holdings.len() {
                ranks[i][rank][suit] = 1;
                num_ranks[i][rank] += 1;
                num_suits[i][suit] += 1;
            }
        }

        for card in combo {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            for i in 0..holdings.len() {
                ranks[i][rank][suit] = 1;
                num_ranks[i][rank] += 1;
                num_suits[i][suit] += 1;
            }
        }

        let hero = HandResult::bare(&ranks[0], &num_ranks[0], &num_suits[0]);
        let vilan = HandResult::bare(&ranks[1], &num_ranks[1], &num_suits[1]);

        if hero > vilan {
            if let Some(count) = stats.get_mut(&usize::from(&hero.hand_rank)) {
                *count += 1;
            } else {
                stats.insert(usize::from(&hero.hand_rank), 1);
            }
        }
        num_combos += 1;
    }
    println!(
        "-> evaluated {} combinations for {}/{} cards",
        num_combos,
        k,
        deck.len()
    );
    println!("{:#?}", stats);
}

fn print_usage() {
    println!("usage: <cmd> [NUM_ITERATIONS] <Holding> <Holding> [COMMUNITY_CARDS..]");
}

fn get_cards(args: &[String]) -> Result<(Vec<Card>, usize, HashSet<Card>)> {
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

    let mut dealt: Vec<Card> = Vec::with_capacity(23); // 9 x holdings + flop + turn + river
    let mut num_players = 0;

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let len = arg.len();

        // holding
        if len == 4 {
            num_players += 1;
            dealt.push(
                deck.take(&Card::from(&arg[..2])?)
                    .ok_or(Error::DuplicateCard)?,
            );
            dealt.push(
                deck.take(&Card::from(&arg[2..])?)
                    .ok_or(Error::DuplicateCard)?,
            );
        }

        // flop
        if len == 3 {
            dealt.push(
                deck.take(&Card::from(&arg[..2])?)
                    .ok_or(Error::DuplicateCard)?,
            );
            dealt.push(
                deck.take(&Card::from(&arg[2..4])?)
                    .ok_or(Error::DuplicateCard)?,
            );
            dealt.push(
                deck.take(&Card::from(&arg[4..6])?)
                    .ok_or(Error::DuplicateCard)?,
            );
        }

        // turn or river
        if len == 2 {
            dealt.push(
                deck.take(&Card::from(&arg[..2])?)
                    .ok_or(Error::DuplicateCard)?,
            );
        }
    }

    Ok((dealt, num_players, deck))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_usage();
        process::exit(1);
    }

    let cmd = &args[1];
    let offset = if cmd == "eval" { 1 } else { 2 };

    let (dealt, num_players, deck) = get_cards(&args[offset..])?;
    let mut holdings: Vec<&[Card]> = Vec::with_capacity(dealt.len());
    for i in 0..num_players {
        let start = i * 2;
        holdings.push(&dealt[start..start + 2]);
    }

    let community_cards = &dealt[num_players * 2..];

    if cmd == "eval" {
        combos(holdings, community_cards, deck, true);
    } else if cmd == "rnd" {
        if let Ok(iterations) = &args[2].parse::<usize>() {
            rnd(holdings, community_cards, deck, *iterations, true);
        } else {
            print_usage();
        }
    }
    Ok(())
}
