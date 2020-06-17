use pkr::prelude::*;
use pkr::error::{Error, Result};

use std::env;
use itertools::Itertools;


// fn handle_result<'a>(hero: &'a Holding, vilan: &'a Holding, benchmark: bool) -> Option<(&'a Holding<'a>, HandResult)> {
// }

fn rnd(num: usize, benchmark: bool) {
    for _ in 0..num {
        let deck = Deck::new();
        let mut hand = Hand::new(&deck, 2);
        let board = hand.board.full();
        let texture = board.texture();

        let hero = hand.get_player(1).as_ref().unwrap();
        let vilan = hand.get_player(2).as_ref().unwrap();

        let hero_result = HandResult::new(hero, &board, &texture);
        let vilan_result = HandResult::new(vilan, &board, &texture);

        let mut winner: &Holding = hero;
        let mut win_rank: Option<&HandRank> = None;
        let mut looser: Option<&HandRank> = None;

        if hero > vilan {
            winner = hero;
            win_rank = Some(&hero_result.hand_rank);
            looser = Some(&vilan_result.hand_rank);
        } else if hero < vilan {
            winner = vilan;
            win_rank = Some(&vilan_result.hand_rank);
            looser = Some(&hero_result.hand_rank);
        }

        if !benchmark {
            let flop = board.flop();
            let turn = board.turn();
            let river = board.river();

            if let Some(rank) = win_rank {
                println!(
                    "{}, {} | {} {} {} | {} | {}\t\t{}\twins\t{:?} over {:?}",
                    hero,
                    vilan,
                    flop[0],
                    flop[1],
                    flop[2],
                    turn,
                    river,
                    winner,
                    rank,
                    looser.unwrap(),
                );
            } else {
                println!(
                    "{}, {} | {} {} {} | {} | {} ¯\\_(ツ)_/¯[split]\t{:?} vs. {:?}",
                    hero, vilan, flop[0], flop[1], flop[2], turn, river, hero_result.hand_rank, vilan_result.hand_rank
                );
            }
        }
    }
}

fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn two_player(args: &Vec<String>) -> Result<()> {
    if args.len() < 3 {
        return Err(Error::InvalidNumberOfArguments);
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    if arg1.len() < 4 || arg2.len() < 4 {
        return Err(Error::InvalidHolding);
    }

    let hero_card1 = Card::from(&arg1[..2])?;
    let hero_card2 = Card::from(&arg1[2..])?;
    let hero_cards = [hero_card1, hero_card2];

    let vilan_card1 = Card::from(&arg2[..2])?;
    let vilan_card2 = Card::from(&arg2[2..])?;
    let vilan_cards = [vilan_card1, vilan_card2];

    let hero_holding = Holding::new(&hero_cards)?;
    let vilan_holding = Holding::new(&vilan_cards)?;

    println!("{} vs. {}", &hero_holding, &vilan_holding);

    let deck_cards = Deck::default().cards.to_vec();
    let cards: Vec<Card> = deck_cards
        .iter()
        .filter_map(|card| {
            if !hero_holding.cards.contains(card) && !vilan_holding.cards.contains(card) {
                // TODO get rid of the `to_owned`
                return Some(card.to_owned());
            }
            None
        }).collect();

    let combos = cards.into_iter().combinations(5);

    let mut num_combos = 0;
    let mut hero_wins = 0.0;
    let mut vilan_wins = 0.0;
    let mut splits = 0.0;

    for (n, cards) in combos.into_iter().enumerate() {
        num_combos = n;
        let board = Board::new(&cards[..]).full();
        let texture = board.texture();

        let hero_result = HandResult::new(&hero_holding, &board, &texture);
        let vilan_result = HandResult::new(&vilan_holding, &board, &texture);

        let flop = board.flop();
        let turn = board.turn();
        let river = board.river();

        let mut winner: &Holding = &hero_holding;
        let mut win_rank: Option<&HandRank> = None;
        let mut looser: Option<&HandRank> = None;

        if hero_result > vilan_result {
            hero_wins += 1.0;
            winner = &hero_holding;
            win_rank = Some(&hero_result.hand_rank);
            looser = Some(&vilan_result.hand_rank);
        } else if hero_result < vilan_result {
            vilan_wins += 1.0;
            winner = &vilan_holding;
            win_rank = Some(&vilan_result.hand_rank);
            looser = Some(&hero_result.hand_rank);
        } else {
            splits += 1.0;
        }

        // if let Some(rank) = win_rank {
        //     println!(
        //         "{}, {} | {} {} {} | {} | {}\t\t{}\twins\t{:?} over {:?}",
        //         hero_holding,
        //         vilan_holding,
        //         flop[0],
        //         flop[1],
        //         flop[2],
        //         turn,
        //         river,
        //         winner,
        //         rank,
        //         looser.unwrap(),
        //     );
        // } else {
        //     println!(
        //         "{}, {} | {} {} {} | {} | {} ¯\\_(ツ)_/¯[split]\t{:?} vs. {:?}",
        //         hero_holding, vilan_holding, flop[0], flop[1], flop[2], turn, river, hero_result.hand_rank, vilan_result.hand_rank
        //     );
        // }

        // if n % 100000 == 0 && n > 0 {
        //     println!("{}", ".");
        // }
    }

    println!("Number of combinations: {}", num_combos);
    println!("hero {:.2?}%; vilan {:.2?}%; splits {:.2?}%", hero_wins * 100.0 / num_combos as f32, vilan_wins * 100.0 / num_combos as f32, splits * 100.0 / num_combos as f32);
    Ok(())
}


fn main() {
    let args = get_args();

    if args.len() > 1 {
        let arg1 = &args[1];
        if arg1 == "benchmark" {
            if args.len() > 2 {
                let num_string = &args[2].to_string();
                let num = num_string.parse::<usize>().expect("Not a number");
                rnd(num, true);
            } else {
                let num = 1_000_000;
                rnd(num, true);
            }
        } else if arg1 == "rnd" {
            if args.len() > 2 {
                let num_string = &args[2].to_string();
                let num = num_string.parse::<usize>().expect("Not a number");
                rnd(num, false);
            } else {
                let num = 10;
                rnd(num, false);
            }
        }
        if let Err(e) = two_player(&args) {
            println!("{:?}", e);
        };
    }
}
