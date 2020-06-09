use pkr::prelude::*;

fn main() {
    for _ in 0..10 {
        let deck = Deck::new();
        let hand = Hand::new(&deck).deal(2);
        let hero = hand.get_player(1).as_ref().unwrap();
        let vilan = hand.get_player(2).as_ref().unwrap();

        let flop = hand.flop();
        let turn = hand.turn();
        let river = hand.river();

        println!(
            "[{}{}] vs. [{}{}] | {}{}{} | {} | {}",
            hero.high_card(),
            hero.low_card(),
            vilan.high_card(),
            vilan.low_card(),
            flop[0],
            flop[1],
            flop[2],
            turn[0],
            river[0]
        );
    }
}
