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

        let mut winner: &Holding = hero;
        let mut win_rank: Option<&HandRank> = None;
        let mut looser: Option<&HandRank> = None;

        if hero_result > vilan_result {
            winner = hero;
            win_rank = Some(&hero_rank);
            looser = Some(&vilan_rank);
        } else if hero_result < vilan_result {
            winner = vilan;
            win_rank = Some(&vilan_rank);
            looser = Some(&hero_rank);
        }

        if let Some(rank) = win_rank {
            println!(
                "{}, {} | {} {} {} | {} | {}\t\t{}\t wins with {:?} over {:?}",
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
                hero, vilan, flop[0], flop[1], flop[2], turn, river, hero_rank, vilan_rank
            );
        }
    }
}
