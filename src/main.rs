use pkr::prelude::*;

// use std::mem;

fn main() {
    let mut count = 0;

    for _ in 0..1_000_000 {
        let deck = Deck::new();
        let holding = Holding::new(&deck.cards[..2]).unwrap();
        let other = Holding::new(&deck.cards[2..4]).unwrap();

        if holding.beats(&other) {
            count += 1;
        };
        // if !(first.is_pocket_pair() || second.is_pocket_pair()) {
        //     continue;
        // }

        // println!(
        //     "{}{} beats {}{} :: {}",
        //     first.high_card(),
        //     first.low_card(),
        //     second.high_card(),
        //     second.low_card(),
        //     first.beats(&second)
        // );
    }

    println!("{} beats!", count);

    // if let Ok(deck) = Deck::new(&mut cards) {
    //     let smb = &deck.get_holding();
    //     // let bb = deck.get_holding();
    //     println!("{:#?}", smb);
    //     // println!("{:#?}", bb);
    // }

    // println!("Card uses {:?} bytes", mem::size_of::<Card>());
    // println!("Rank uses {:?} bytes", mem::size_of::<Rank>());
    // println!("Suit uses {:?} bytes", mem::size_of::<Suit>());
    // println!("Deck uses {:?} bytes", mem::size_of::<[Card; 52]>());

    // let first = Card::new(Rank::Ace, Suit::Clubs);
    // let second = Card::new(Rank::Ace, Suit::Spades);
    // println!("{:?}", first);
    // let hand = Holding::new(first, second).unwrap();
    // println!("{:?}", hand);
    // let hand = Holding::from("AsKs").unwrap();
    // println!("{:?}", hand);

    // for (idx, card) in Deck::new().into_iter().enumerate() {
    //     println!("{}: {:?}", idx + 1, card);
    // }
}
