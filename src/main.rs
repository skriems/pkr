use pkr::prelude::*;

fn main() {

    for _ in 0..100 {
        let deck = Deck::new();
        let mut hand = Hand::new(&deck, 2);
        let board = hand.board.full();
        let texture = board.texture();

        let hero = hand.get_player(1).as_ref().unwrap();
        let vilan = hand.get_player(2).as_ref().unwrap();

        let flop = board.flop();
        let turn = board.turn();
        let river = board.river();

        let hero_result = HandResult::new(hero, &board, &texture);
        let hero_rank = hero_result.rank();
        let vilan_result = HandResult::new(vilan, &board, &texture);
        let vilan_rank = vilan_result.rank();

        let mut winner: Option<&Holding> = None;
        let mut winner_rank: Option<&HandRank> = None;

        if hero_rank > vilan_rank {
            winner = Some(hero);
            winner_rank = Some(&hero_rank);
        } else if hero_rank < vilan_rank {
            winner = Some(vilan);
            winner_rank = Some(&vilan_rank);
        } else if hero_rank == vilan_rank {

            // split with TwoPair by high_card
            if hero_rank == HandRank::TwoPair {
                if hero_result.high_card > vilan_result.high_card {
                    winner = Some(hero);
                    winner_rank = Some(&hero_rank);
                } else if hero_result.high_card < vilan_result.high_card {
                    winner = Some(vilan);
                    winner_rank = Some(&vilan_rank);
                }
            }
        }

        if let Some(player) = winner {
            println!(
                "{}, {} | {} {} {} | {} | {}\t{} wins with {:?}",
                hero,
                vilan,
                flop[0],
                flop[1],
                flop[2],
                turn,
                river,
                player,
                winner_rank.unwrap()
            );
        } else {
            println!(
                "{}, {} | {} {} {} | {} | {}\t ¯\\_(ツ)_/¯ {:?} vs. {:?}",
                hero,
                vilan,
                flop[0],
                flop[1],
                flop[2],
                turn,
                river,
                hero_rank,
                vilan_rank
            );
        }
    }
}
