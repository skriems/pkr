use pkr::prelude::*;

use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::env;
use std::process;

fn get_pool<'a>(cards: &'a Vec<Card>, dealt: &'a Vec<Card>) -> Vec<&'a Card> {
    cards
        .iter()
        .filter_map(|card| {
            if dealt.contains(card) {
                return None;
            }
            Some(card)
        })
        .collect()
}


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

fn rnd(deck: &Vec<Card>, dealt: &Vec<Card>, num_players: usize, iterations: usize, benchmark: bool) {
    let mut holdings: Vec<&[Card]> = vec![];

    for i in 0..num_players {
        let start = i * 2;
        holdings.push(&dealt[start..start + 2]);
    }

    let community_cards = &dealt[num_players * 2..];

    let mut remaining: Vec<&Card> = deck.iter().filter(|c| !dealt.contains(c)).collect();

    let mut rng = ThreadRng::default();

    // Stats
    let mut num_combos = 0;
    let mut hero_wins = 0.0;
    let mut vilan_wins = 0.0;
    let mut splits = 0.0;
    let k = 5 - community_cards.len();

    while num_combos < iterations {
        remaining.shuffle(&mut rng);
        let combo = &remaining[..k];

        let hero_matrix = Matrix::with_slice(holdings[0], &community_cards, &combo);
        let hero = HandResult::new(&hero_matrix);

        let vilan_matrix = Matrix::with_slice(holdings[1], &community_cards, &combo);
        let vilan = HandResult::new(&vilan_matrix);

        if hero > vilan {
            hero_wins += 1.0;
            if !benchmark {
                print_result(&hero, &hero_matrix.cards, &vilan, &vilan_matrix.cards);
            }
        } else if hero < vilan {
            vilan_wins += 1.0;
            if !benchmark {
                print_result(&vilan, &vilan_matrix.cards, &hero, &hero_matrix.cards);
            }
        } else {
            splits += 1.0;
            // if !benchmark {
            //     print_result(&hero, &hero_matrix.cards, &vilan, &vilan_matrix.cards);
            // }
        }
        num_combos += 1;
    }
    println!("-> evaluated {} random hands", num_combos);
    println!(
        "hero {:.2?}%; vilan {:.2?}%; splits {:.2?}%",
        hero_wins * 100.0 / num_combos as f32,
        vilan_wins * 100.0 / num_combos as f32,
        splits * 100.0 / num_combos as f32
    );
}

fn combos(deck: &Vec<Card>, dealt: &Vec<Card>, num_players: usize, benchmark: bool) {

    let mut holdings: Vec<&[Card]> = vec![];

    for i in 0..num_players {
        let start = i * 2;
        holdings.push(&dealt[start..start + 2]);
    }

    let community_cards = &dealt[num_players * 2..];

    let pool = get_pool(&deck, &dealt);

    // Stats
    let mut num_combos = 0;
    let mut hero_wins = 0.0;
    let mut vilan_wins = 0.0;
    let mut splits = 0.0;
    let k = 5 - community_cards.len();

    for combo in pool.iter().combinations(k) {

        let hero_matrix = Matrix::with_combo(holdings[0], &community_cards, &combo);
        let hero = HandResult::new(&hero_matrix);

        let vilan_matrix = Matrix::with_combo(holdings[1], &community_cards, &combo);
        let vilan = HandResult::new(&vilan_matrix);

        if hero > vilan {
            hero_wins += 1.0;
            if !benchmark {
                print_result(&hero, &hero_matrix.cards, &vilan, &vilan_matrix.cards);
            }
        } else if hero < vilan {
            vilan_wins += 1.0;
            if !benchmark {
                print_result(&vilan, &vilan_matrix.cards, &hero, &hero_matrix.cards);
            }
        } else {
            splits += 1.0;
            // if !benchmark {
            //     print_result(&hero, &hero_matrix.cards, &vilan, &vilan_matrix.cards);
            // }
        }
        num_combos += 1;
    }
    println!("-> evaluated {} combinations for {}/{} cards", num_combos, k, pool.len());
    println!(
        "hero {:.2?}%; vilan {:.2?}%; splits {:.2?}%",
        hero_wins * 100.0 / num_combos as f32,
        vilan_wins * 100.0 / num_combos as f32,
        splits * 100.0 / num_combos as f32
    );
}

fn get_cards(args: &[String]) -> Vec<Card> {

    let mut dealt: Vec<Card> = vec![];

    if args.len() > 1 {
        dealt.push(Card::from(&args[1][..2]).unwrap());
        dealt.push(Card::from(&args[1][2..]).unwrap());
    }
    if args.len() > 2 {
        dealt.push(Card::from(&args[2][..2]).unwrap());
        dealt.push(Card::from(&args[2][2..]).unwrap());
    }
    // Flop
    if args.len() > 3 {
        let arg = &args[3];
        if arg.len() >= 2 {
            dealt.push(Card::from(&args[3][..2]).unwrap());
        }
        if arg.len() >= 4 {
            dealt.push(Card::from(&args[3][2..4]).unwrap());
        }
        if arg.len() == 6 {
            dealt.push(Card::from(&args[3][4..]).unwrap());
        }
    }
    dealt
}

fn print_usage() {
    println!("usage: <cmd> [NUM_ITERATIONS] <Holding> <Holding> [COMMUNITY_CARDS..]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_usage();
        process::exit(1);
    }

    let deck = vec![
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
        Card::new(Rank::Nine, Suit::Clubs),
        Card::new(Rank::Eight, Suit::Clubs),
        Card::new(Rank::Seven, Suit::Clubs),
        Card::new(Rank::Six, Suit::Clubs),
        Card::new(Rank::Five, Suit::Clubs),
        Card::new(Rank::Four, Suit::Clubs),
        Card::new(Rank::Three, Suit::Clubs),
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::King, Suit::Spades),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Jack, Suit::Spades),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Nine, Suit::Spades),
        Card::new(Rank::Eight, Suit::Spades),
        Card::new(Rank::Seven, Suit::Spades),
        Card::new(Rank::Six, Suit::Spades),
        Card::new(Rank::Five, Suit::Spades),
        Card::new(Rank::Four, Suit::Spades),
        Card::new(Rank::Three, Suit::Spades),
        Card::new(Rank::Two, Suit::Spades),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Nine, Suit::Hearts),
        Card::new(Rank::Eight, Suit::Hearts),
        Card::new(Rank::Seven, Suit::Hearts),
        Card::new(Rank::Six, Suit::Hearts),
        Card::new(Rank::Five, Suit::Hearts),
        Card::new(Rank::Four, Suit::Hearts),
        Card::new(Rank::Three, Suit::Hearts),
        Card::new(Rank::Two, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Diamonds),
        Card::new(Rank::King, Suit::Diamonds),
        Card::new(Rank::Queen, Suit::Diamonds),
        Card::new(Rank::Jack, Suit::Diamonds),
        Card::new(Rank::Ten, Suit::Diamonds),
        Card::new(Rank::Nine, Suit::Diamonds),
        Card::new(Rank::Eight, Suit::Diamonds),
        Card::new(Rank::Seven, Suit::Diamonds),
        Card::new(Rank::Six, Suit::Diamonds),
        Card::new(Rank::Five, Suit::Diamonds),
        Card::new(Rank::Four, Suit::Diamonds),
        Card::new(Rank::Three, Suit::Diamonds),
        Card::new(Rank::Two, Suit::Diamonds),
    ];

    let cmd = &args[1];
    if cmd == "eval" {
        let dealt = get_cards(&args[1..]);
        combos(&deck, &dealt, 2, true);
    } else if cmd == "rnd" {
        if let Ok(iterations) = &args[2].parse::<usize>() {
            let dealt = get_cards(&args[2..]);
            rnd(&deck, &dealt, 2, *iterations, true);
        } else {
            print_usage();
        }
    }
}
