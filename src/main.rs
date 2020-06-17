use pkr::error::{Error, Result};
use pkr::prelude::*;

use itertools::Itertools;
use std::env;

// fn print_split(hero: &HandResult, vilan: &HandResult) {
//     let flop = hero.board.flop();
//     let turn = hero.board.turn();
//     let river = hero.board.river();

//     println!(
//         "{}, {} | {} {} {} | {} | {} ¯\\_(ツ)_/¯[split]\t{:?} vs. {:?}",
//         hero.holding,
//         vilan.holding,
//         flop[0],
//         flop[1],
//         flop[2],
//         turn,
//         river,
//         hero.hand_rank,
//         vilan.hand_rank
//     );
// }

// fn print_result(winner: &HandResult, looser: &HandResult) {
//     let flop = winner.board.flop();
//     let turn = winner.board.turn();
//     let river = winner.board.river();

//     println!(
//         "{}, {} | {} {} {} | {} | {}\t\t{}\twins\t{:?} over {:?}",
//         winner.holding,
//         looser.holding,
//         flop[0],
//         flop[1],
//         flop[2],
//         turn,
//         river,
//         winner.holding,
//         winner.hand_rank,
//         looser.hand_rank,
//     );
// }

fn print_result(
    winner: &HandResult,
    winner_cards: &Vec<Card>,
    looser: &HandResult,
    looser_cards: &Vec<Card>,
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

// fn rnd(num: usize, benchmark: bool) {
//     for _ in 0..num {
//         let deck = Deck::new();
//         let mut hand = Hand::new(&deck, 2);
//         let board = hand.board.full();
//         let texture = board.texture();

//         let hero_holding = hand.get_player(1).as_ref().unwrap();
//         let vilan_holding = hand.get_player(2).as_ref().unwrap();

//         let hero = HandResult::new(hero_holding, &board, &texture);
//         let vilan = HandResult::new(vilan_holding, &board, &texture);

//         if hero > vilan {
//             if !benchmark {
//                 print_result(&hero, &vilan);
//             }
//         } else if hero < vilan {
//             if !benchmark {
//                 print_result(&vilan, &hero);
//             }
//         } else {
//             if !benchmark {
//                 print_split(&hero, &vilan);
//             }
//         }
//     }
// }

fn get_args() -> Vec<String> {
    return env::args().collect();
}

// fn two_player(args: &Vec<String>, benchmark: bool) -> Result<()> {
//     if args.len() < 3 {
//         return Err(Error::InvalidNumberOfArguments);
//     }

//     let arg1 = &args[1];
//     let arg2 = &args[2];

//     let hero_card1 = Card::from(&arg1[..2])?;
//     let hero_card2 = Card::from(&arg1[2..])?;
//     let hero_cards = [hero_card1, hero_card2];

//     let vilan_card1 = Card::from(&arg2[..2])?;
//     let vilan_card2 = Card::from(&arg2[2..])?;
//     let vilan_cards = [vilan_card1, vilan_card2];

//     let hero_holding = Holding::new(&hero_cards)?;
//     let vilan_holding = Holding::new(&vilan_cards)?;

//     println!("{} vs. {}", &hero_holding, &vilan_holding);

//     let deck_cards = Deck::default().cards.to_vec();
//     let cards: Vec<Card> = deck_cards
//         .iter()
//         .filter_map(|card| {
//             if !hero_holding.cards.contains(card) && !vilan_holding.cards.contains(card) {
//                 // TODO get rid of the `to_owned`
//                 return Some(card.to_owned());
//             }
//             None
//         })
//         .collect();

//     let combos = cards.into_iter().combinations(5);

//     let mut num_combos = 0;
//     let mut hero_wins = 0.0;
//     let mut vilan_wins = 0.0;
//     let mut splits = 0.0;

//     for (n, cards) in combos.into_iter().enumerate() {
//         num_combos = n;
//         let board = Board::new(&cards).full();
//         let texture = board.texture();

//         let hero = HandResult::new(&hero_holding, &board, &texture);
//         let vilan = HandResult::new(&vilan_holding, &board, &texture);

//         if hero > vilan {
//             hero_wins += 1.0;

//             if !benchmark {
//                 print_result(&hero, &vilan);
//             }
//         } else if hero < vilan {
//             vilan_wins += 1.0;

//             if !benchmark {
//                 print_result(&vilan, &hero);
//             }
//         } else {
//             splits += 1.0;

//             if !benchmark {
//                 print_split(&hero, &vilan);
//             }
//         }
//     }

//     println!("-> evaluated {} combinations", num_combos);
//     println!(
//         "hero {:.2?}%; vilan {:.2?}%; splits {:.2?}%",
//         hero_wins * 100.0 / num_combos as f32,
//         vilan_wins * 100.0 / num_combos as f32,
//         splits * 100.0 / num_combos as f32
//     );
//     Ok(())
// }

struct Matrix {
    pub cards: Vec<Card>,
    pub ranks: [[usize; 4]; 13],
    pub num_ranks: [usize; 13],
    pub num_suits: [usize; 4],
}

impl Matrix {
    pub fn new(cards: Vec<Card>) -> Self {
        let mut ranks = [
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
        ];

        let mut num_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut num_suits = [0, 0, 0, 0];

        for card in &cards {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        Matrix {
            cards,
            ranks,
            num_ranks,
            num_suits,
        }
    }
}

fn test_vec(args: Vec<String>, benchmark: bool) {
    let mut dealt: Vec<Card> = vec![];
    let mut hero_cards: Vec<Card> = vec![];
    let mut vilan_cards: Vec<Card> = vec![];
    let mut k = 5;

    if args.len() > 1 {
        let hero_card1 = Card::from(&args[1][..2]).unwrap();
        let hero_card2 = Card::from(&args[1][2..]).unwrap();
        hero_cards.push(hero_card1);
        hero_cards.push(hero_card2);
        dealt.push(hero_card1);
        dealt.push(hero_card2);
    }
    if args.len() > 2 {
        let vilan_card1 = Card::from(&args[2][..2]).unwrap();
        let vilan_card2 = Card::from(&args[2][2..]).unwrap();
        vilan_cards.push(vilan_card1);
        vilan_cards.push(vilan_card2);
        dealt.push(vilan_card1);
        dealt.push(vilan_card2);
    }
    if args.len() > 3 {
        let arg = &args[3];
        if arg.len() >= 2 {
            dealt.push(Card::from(&args[3][..2]).unwrap());
            k -= 1;
        }
        if arg.len() >= 4 {
            dealt.push(Card::from(&args[3][2..4]).unwrap());
            k -= 1;
        }
        if arg.len() == 6 {
            dealt.push(Card::from(&args[3][4..]).unwrap());
            k -= 1;
        }
    }

    let hero_matrix = Matrix::new(hero_cards);
    let vilan_matrix = Matrix::new(vilan_cards);

    let cards = vec![
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

    let pool: Vec<&Card> = cards
        .iter()
        .filter_map(|card| {
            if dealt.contains(card) {
                return None;
            }
            Some(card)
        })
        .collect();

    // Stats
    let mut num_combos = 0;
    let mut hero_wins = 0.0;
    let mut vilan_wins = 0.0;
    let mut splits = 0.0;

    for combo in pool.iter().combinations(k) {
        let mut ranks = hero_matrix.ranks;
        let mut num_ranks = hero_matrix.num_ranks;
        let mut num_suits = hero_matrix.num_suits;

        for card in &combo {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        let hero = HandResult::new(&ranks, &num_ranks, &num_suits);

        let mut ranks = vilan_matrix.ranks;
        let mut num_ranks = vilan_matrix.num_ranks;
        let mut num_suits = vilan_matrix.num_suits;

        for card in &combo {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            ranks[rank][suit] = 1;
            num_ranks[rank] += 1;
            num_suits[suit] += 1;
        }

        let vilan = HandResult::new(&ranks, &num_ranks, &num_suits);

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
            if !benchmark {
                print_result(&hero, &hero_matrix.cards, &vilan, &vilan_matrix.cards);
            }
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

fn main() {
    let args = get_args();
    test_vec(args, true);

    // if args.len() > 1 {
    //     let arg1 = &args[1];
    //     if arg1 == "benchmark" {
    //         if args.len() > 2 {
    //             let num_string = &args[2].to_string();
    //             let num = num_string.parse::<usize>().expect("Not a number");
    //             rnd(num, true);
    //         } else {
    //             let num = 1_000_000;
    //             rnd(num, true);
    //         }
    //     } else if arg1 == "rnd" {
    //         if args.len() > 2 {
    //             let num_string = &args[2].to_string();
    //             let num = num_string.parse::<usize>().expect("Not a number");
    //             rnd(num, false);
    //         } else {
    //             let num = 10;
    //             rnd(num, false);
    //         }
    //     }
    //     if let Err(e) = two_player(&args, true) {
    //         println!("{:?}", e);
    //     };
    // }
}
