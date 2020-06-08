use pkr::prelude::*;

fn main() {
    for _ in 0..1 {
        let hand = Hand::new();
        let hero = Holding::new(&hand.deck[..2]).unwrap();
        let vilan = Holding::new(&hand.deck[2..4]).unwrap();

        let mut iter = hand.deck[4..].iter();
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
}
