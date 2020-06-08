use pkr::prelude::*;

// use std::mem;

fn main() {
    for _ in 0..1 {
        let deck = Deck::new();
        let hero = Holding::new(&deck.cards[..2]).unwrap();
        let vilan = Holding::new(&deck.cards[2..4]).unwrap();

        let mut iter = deck.cards[4..].iter();
        let flop_1 = iter.next();
        let flop_2 = iter.next();
        let flop_3 = iter.next();
        let _ = iter.next();
        let turn = iter.next();
        let _ = iter.next();
        let river = iter.next();

        println!(
            "[{}{}] vs. [{}{}] | {}{}{} | {} | {}",
            hero.high_card(),
            hero.low_card(),
            vilan.high_card(),
            vilan.low_card(),
            flop_1.unwrap(),
            flop_2.unwrap(),
            flop_3.unwrap(),
            turn.unwrap(),
            river.unwrap()
        );
    }

    // println!("Card uses {:?} bytes", mem::size_of::<Card>());
    // println!("Rank uses {:?} bytes", mem::size_of::<Rank>());
    // println!("Suit uses {:?} bytes", mem::size_of::<Suit>());
    // println!("Deck uses {:?} bytes", mem::size_of::<[Card; 52]>());
}
