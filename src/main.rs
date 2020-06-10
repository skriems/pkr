use pkr::prelude::*;

fn main() {

    for _ in 0..100 {
        let deck = Deck::new();
        let mut hand = Hand::new(&deck, 2);
        let board = hand.board.full();

        let hero = hand.get_player(1).as_ref().unwrap();
        let vilan = hand.get_player(2).as_ref().unwrap();

        let flop = board.flop();
        let turn = board.turn();
        let river = board.river();

        let rank1 = HandResult::new(hero, &board).rank();
        let rank2 = HandResult::new(vilan, &board).rank();

        if rank1 > rank2 {
            println!(
                "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                hero,
                vilan,
                flop[0],
                flop[1],
                flop[2],
                turn,
                river,
                hero,
                rank1
            );
        } else if rank2 > rank1 {
            println!(
                "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                hero,
                vilan,
                flop[0],
                flop[1],
                flop[2],
                turn,
                river,
                vilan,
                rank2 
            );
        } else if rank1 == HandRank::HighCard && rank2 == HandRank::HighCard {
            if hero.beats(vilan) {
                println!(
                    "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                    hero,
                    vilan,
                    flop[0],
                    flop[1],
                    flop[2],
                    turn,
                    river,
                    hero,
                    rank1,
                );
            } else if vilan.beats(hero) {
                println!(
                    "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                    hero,
                    vilan,
                    flop[0],
                    flop[1],
                    flop[2],
                    turn,
                    river,
                    vilan,
                    rank2
                );
            }
        } else {
            println!(
                "{}, {} | {} {} {} | {} | {}",
                hero,
                vilan,
                flop[0],
                flop[1],
                flop[2],
                turn,
                river
            );
        }
    }
}
