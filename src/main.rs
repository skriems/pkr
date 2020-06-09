use pkr::prelude::*;

fn main() {

    for _ in 0..100 {
        let deck = Deck::new();
        let hand = Hand::new(&deck).deal(2);
        let hero = hand.get_player(1).as_ref().unwrap();
        let vilan = hand.get_player(2).as_ref().unwrap();

        let flop = hand.flop();
        let turn = hand.turn();
        let river = hand.river();

        let res1 = HandResult::new(hero, Some(&flop), Some(&turn), Some(&river));
        let res2 = HandResult::new(vilan, Some(&flop), Some(&turn), Some(&river));

        if res1.rank > res2.rank {
            println!(
                "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                hero,
                vilan,
                flop.cards[0],
                flop.cards[1],
                flop.cards[2],
                turn,
                river,
                hero,
                res1.rank
            );
        } else if res2.rank > res1.rank {
            println!(
                "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                hero,
                vilan,
                flop.cards[0],
                flop.cards[1],
                flop.cards[2],
                turn,
                river,
                vilan,
                res2.rank
            );
        } else if res1.rank == HandRank::HighCard && res2.rank == HandRank::HighCard {
            if hero.beats(vilan) {
                println!(
                    "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                    hero,
                    vilan,
                    flop.cards[0],
                    flop.cards[1],
                    flop.cards[2],
                    turn,
                    river,
                    hero,
                    res1.rank
                );
            } else if vilan.beats(hero) {
                println!(
                    "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                    hero,
                    vilan,
                    flop.cards[0],
                    flop.cards[1],
                    flop.cards[2],
                    turn,
                    river,
                    vilan,
                    res2.rank
                );
            }
        } else {
            println!(
                "{}, {} | {} {} {} | {} | {}",
                hero,
                vilan,
                flop.cards[0],
                flop.cards[1],
                flop.cards[2],
                turn,
                river
            );
        }
    }
}
