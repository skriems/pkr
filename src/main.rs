use pkr::prelude::*;
use pkr::error::{Error, Result};

use std::env;
use itertools::Itertools;

fn print_split(hero: &HandResult, vilan: &HandResult) {
    let flop = hero.board.flop();
    let turn = hero.board.turn();
    let river = hero.board.river();

    println!(
        "{}, {} | {} {} {} | {} | {} ¯\\_(ツ)_/¯[split]\t{:?} vs. {:?}",
        hero.holding, vilan.holding, flop[0], flop[1], flop[2], turn, river, hero.hand_rank, vilan.hand_rank
    );
}

fn print_result(winner: &HandResult, looser: &HandResult) {
    let flop = winner.board.flop();
    let turn = winner.board.turn();
    let river = winner.board.river();

    println!(
        "{}, {} | {} {} {} | {} | {}\t\t{}\twins\t{:?} over {:?}",
        winner.holding,
        looser.holding,
        flop[0],
        flop[1],
        flop[2],
        turn,
        river,
        winner.holding,
        winner.hand_rank,
        looser.hand_rank,
    );
}

fn rnd(num: usize, benchmark: bool) {
    for _ in 0..num {
        let deck = Deck::new();
        let mut hand = Hand::new(&deck, 2);
        let board = hand.board.full();
        let texture = board.texture();

        let hero_holding = hand.get_player(1).as_ref().unwrap();
        let vilan_holding = hand.get_player(2).as_ref().unwrap();

        let hero = HandResult::new(hero_holding, &board, &texture);
        let vilan = HandResult::new(vilan_holding, &board, &texture);

        if hero > vilan {
            if !benchmark {
                print_result(&hero, &vilan);
            }
        } else if hero < vilan {
            if !benchmark {
                print_result(&vilan, &hero);
            }
        } else {
            if !benchmark {
                print_split(&hero, &vilan);
            }
        }
    }
}

fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn two_player(args: &Vec<String>, benchmark: bool) -> Result<()> {
    if args.len() < 3 {
        return Err(Error::InvalidNumberOfArguments);
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

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
        let board = Board::new(&cards).full();
        let texture = board.texture();

        let hero = HandResult::new(&hero_holding, &board, &texture);
        let vilan = HandResult::new(&vilan_holding, &board, &texture);

        if hero > vilan {
            hero_wins += 1.0;

            if !benchmark {
                print_result(&hero, &vilan);
            }
        } else if hero < vilan {
            vilan_wins += 1.0;

            if !benchmark {
                print_result(&vilan, &hero);
            }
        } else {
            splits += 1.0;

            if !benchmark {
                print_split(&hero, &vilan);
            }
        }
    }

    println!("-> evaluated {} combinations", num_combos);
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
        if let Err(e) = two_player(&args, true) {
            println!("{:?}", e);
        };
    }
}
