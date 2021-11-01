use pkr::prelude::*;

#[test]
fn high_card() {
    let combo: Vec<&Card> = vec![];
    // [J♦ T♣], [7❤ 4♦] | Q♦ J♠ 2❤ | T❤ | 9♦
    let holdings = [Card::from("7h").unwrap(), Card::from("4d").unwrap()];
    let community_cards = [
        Card::from("Qd").unwrap(),
        Card::from("Js").unwrap(),
        Card::from("2h").unwrap(),
        Card::from("Th").unwrap(),
        Card::from("9d").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::HighCard);

    // [K♣ T♦], [K♦ 8♠] | 8♦ 2♣ 6♦ | 9♠ | 5♣	¯\_(ツ)_/¯ Pair vs. Pair
    let holdings = [Card::from("Kc").unwrap(), Card::from("Td").unwrap()];
    let community_cards = [
        Card::from("8d").unwrap(),
        Card::from("2c").unwrap(),
        Card::from("6d").unwrap(),
        Card::from("9s").unwrap(),
        Card::from("5c").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::HighCard);

    //[K❤ 8♦], [9❤ 8❤] | J♠ 7♦ 2❤ | K♦ | 5♦	¯\_(ツ)_/¯ Pair vs. Pair
    let holdings = [Card::from("9h").unwrap(), Card::from("8h").unwrap()];
    let community_cards = [
        Card::from("Js").unwrap(),
        Card::from("7d").unwrap(),
        Card::from("2h").unwrap(),
        Card::from("Kd").unwrap(),
        Card::from("5d").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::HighCard);
}

#[test]
fn high_card_vs_high_card() {
    let combo: Vec<&Card> = vec![];
    // [A♦ 3♣], [A❤ 8♣] | T♠ 5♠ 9♠ | K♦ | Q♦	¯\_(ツ)_/¯ HighCard vs. HighCard
    let community_cards = [
        Card::from("Ts").unwrap(),
        Card::from("5s").unwrap(),
        Card::from("9s").unwrap(),
        Card::from("Kd").unwrap(),
        Card::from("Qd").unwrap(),
    ];
    let holdings = [Card::from("Ad").unwrap(), Card::from("3c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    let holdings = [Card::from("Ah").unwrap(), Card::from("8c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::HighCard);
    assert_eq!(hand2.rank, HandRank::HighCard);
    // both have AKQT9 -> split!
    assert_eq!(hand1 == hand2, true);
}

#[test]
fn pair_flop() {
    let combo: Vec<&Card> = vec![];
    let holdings = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
    let community_cards = [
        Card::from("7c").unwrap(),
        Card::from("2s").unwrap(),
        Card::from("Kd").unwrap(),
        Card::from("5d").unwrap(),
        Card::from("3c").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));

    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Pair(Rank::King));
}

#[test]
fn pair_vs_pair() {
    let combo: Vec<&Card> = vec![];
    // [6♠ 4❤], [K♦ 7❤] | J♦ A♠ 8♦ | 8♣ | 2❤	¯\_(ツ)_/¯ HighCard vs. HighCard
    let community_cards = [
        Card::from("Jd").unwrap(),
        Card::from("As").unwrap(),
        Card::from("8d").unwrap(),
        Card::from("8c").unwrap(),
        Card::from("2h").unwrap(),
    ];
    let holdings = [Card::from("Kd").unwrap(), Card::from("7h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    let holdings = [Card::from("6s").unwrap(), Card::from("4h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Pair(Rank::Eight));
    assert_eq!(hand2.rank, HandRank::Pair(Rank::Eight));
    assert_eq!(hand1 > hand2, true);
}

#[test]
fn two_pair() {
    let combo: Vec<&Card> = vec![];
    // [T♦ 3♠], [Q♣ 3♦] | 8♣ 3❤ 7♣ | 9♣ | Q♦	¯\_(ツ)_/¯ Pair vs. Pair
    let community_cards = [
        Card::from("8c").unwrap(),
        Card::from("3h").unwrap(),
        Card::from("7c").unwrap(),
        Card::from("9c").unwrap(),
        Card::from("Qd").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("3d").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::TwoPair(Rank::Queen, Rank::Three));
}

#[test]
fn two_pairs_paired_board_and_river() {
    let combo: Vec<&Card> = vec![];
    // [A♠ K♣], [A♣ 7♣] | 4♦ 3♠ 3♦ | 6♠ | K❤    [A♠ K♣] wins with TwoPair
    let community_cards = [
        Card::from("4d").unwrap(),
        Card::from("3s").unwrap(),
        Card::from("3d").unwrap(),
        Card::from("6s").unwrap(),
        Card::from("Kh").unwrap(),
    ];

    let holdings = [Card::from("As").unwrap(), Card::from("Kc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::TwoPair(Rank::King, Rank::Three));
}

#[test]
fn two_pair_vs_two_pair() {
    let combo: Vec<&Card> = vec![];
    // [7♣ 4♦], [9♠ 3♦] | 6❤ 4♣ 6♠ | J♣ | 3♣	¯\_(ツ)_/¯ TwoPair vs. TwoPair
    let community_cards = [
        Card::from("6h").unwrap(),
        Card::from("4c").unwrap(),
        Card::from("6s").unwrap(),
        Card::from("Jc").unwrap(),
        Card::from("3c").unwrap(),
    ];
    let holdings = [Card::from("7c").unwrap(), Card::from("4d").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::TwoPair(Rank::Six, Rank::Four));
    // assert_eq!(hand1.num_ranks, &[0, 1, 2, 0, 2, 1, 0, 0, 0, 1, 0, 0, 0]);

    let holdings = [Card::from("9s").unwrap(), Card::from("3d").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand2.rank, HandRank::TwoPair(Rank::Six, Rank::Three));

    // 64 vs 63
    assert_eq!(hand1 > hand2, true);

    // [K♠ Q♣], [9♦ 3❤] | 4♠ Q♦ 7❤ | 4♣ | 9❤	[9♦ 3❤] wins with TwoPair
    let community_cards = [
        Card::from("4s").unwrap(),
        Card::from("Qd").unwrap(),
        Card::from("7h").unwrap(),
        Card::from("4c").unwrap(),
        Card::from("9h").unwrap(),
    ];

    let holdings = [Card::from("Ks").unwrap(), Card::from("Qc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand1.rank, HandRank::TwoPair(Rank::Queen, Rank::Four));

    let holdings = [Card::from("9d").unwrap(), Card::from("3h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand2.rank, HandRank::TwoPair(Rank::Nine, Rank::Four));

    // TwoPair(Q4) > TwoPair(94)
    assert_eq!(hand1 > hand2, true);

    // [T♠ 5♣], [A♦ 3♣] | A♣ 6♦ 8♣ | 8♦ | 6♣	¯\_(ツ)_/¯ TwoPair vs. TwoPair
    let community_cards = [
        Card::from("Ac").unwrap(),
        Card::from("6d").unwrap(),
        Card::from("8c").unwrap(),
        Card::from("8d").unwrap(),
        Card::from("6c").unwrap(),
    ];

    let holdings = [Card::from("Ts").unwrap(), Card::from("5c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand1.rank, HandRank::TwoPair(Rank::Eight, Rank::Six));

    let holdings = [Card::from("Ad").unwrap(), Card::from("3c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand2.rank, HandRank::TwoPair(Rank::Ace, Rank::Eight));
    assert_eq!(hand1 > hand2, false);
    // TwoPair(A8) > TwoPair(86)
    assert_eq!(hand1 < hand2, true);
}

#[test]
fn two_pairs_with_pocket_pairs() {
    let combo: Vec<&Card> = vec![];
    // [Q♦ 5♠], [6♣ 6♦] | T❤ 8♣ 8♠ | K♣ | 4♠	[6♣ 6♦] wins with Pair
    let holdings = [Card::from("6c").unwrap(), Card::from("6d").unwrap()];

    let community_cards = [
        Card::from("Th").unwrap(),
        Card::from("8c").unwrap(),
        Card::from("8s").unwrap(),
        Card::from("Kc").unwrap(),
        Card::from("4s").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::TwoPair(Rank::Eight, Rank::Six));
}

#[test]
fn trips_runner_runner() {
    let combo: Vec<&Card> = vec![];
    // [T♦ 8♣], [9♠ 5♦] | 2♠ 4♦ T♣ | 9❤ | 9♦	[9♠ 5♦] wins with FullHouse
    let holdings = [Card::from("9s").unwrap(), Card::from("5d").unwrap()];

    let community_cards = [
        Card::from("2s").unwrap(),
        Card::from("4d").unwrap(),
        Card::from("Tc").unwrap(),
        Card::from("9h").unwrap(),
        Card::from("9d").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Trips(Rank::Nine));
}

#[test]
fn trips_on_turn_high_card() {
    let combo: Vec<&Card> = vec![];
    // [9♣ 3♠], [8♠ 2♦] | T♠ 9❤ J❤ | 9♦ | K❤	[9♣ 3♠] wins with FullHouse
    let holdings = [Card::from("9c").unwrap(), Card::from("3s").unwrap()];

    let community_cards = [
        Card::from("Ts").unwrap(),
        Card::from("9h").unwrap(),
        Card::from("Jh").unwrap(),
        Card::from("9d").unwrap(),
        Card::from("Kh").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Trips(Rank::Nine));
}

#[test]
fn trips_on_turn_low_card() {
    let combo: Vec<&Card> = vec![];
    // [9♣ 3♠], [8♠ 2♦] | T♠ 3❤ J❤ | 3♦ | K❤	[9♣ 3♠] wins with FullHouse
    let holdings = [Card::from("9c").unwrap(), Card::from("3s").unwrap()];

    let community_cards = [
        Card::from("Ts").unwrap(),
        Card::from("3h").unwrap(),
        Card::from("Jh").unwrap(),
        Card::from("3d").unwrap(),
        Card::from("Kh").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Trips(Rank::Three));
}

#[test]
fn trips_on_river_a6() {
    let combo: Vec<&Card> = vec![];
    // [5♠ 3♠], [A❤ 6❤ ] | 4♠ A♣ 3❤ | T♣ | A♦	[A❤ 6❤] wins with FullHouse
    let holdings = [Card::from("Ah").unwrap(), Card::from("6h").unwrap()];

    let community_cards = [
        Card::from("4s").unwrap(),
        Card::from("Ac").unwrap(),
        Card::from("3h").unwrap(),
        Card::from("Tc").unwrap(),
        Card::from("Ad").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Trips(Rank::Ace));
}

#[test]
fn trips_on_river_kj() {
    let combo: Vec<&Card> = vec![];
    // [K♠ J♦], [7♦ 6♠] | K♦ 8❤ 5♠ | 4♠ | K♣	[K♠ J♦] wins with FullHouse
    let holdings = [Card::from("Ks").unwrap(), Card::from("Jd").unwrap()];

    let community_cards = [
        Card::from("Kd").unwrap(),
        Card::from("8h").unwrap(),
        Card::from("5s").unwrap(),
        Card::from("4s").unwrap(),
        Card::from("Ks").unwrap(),
    ];

    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Trips(Rank::King));
}

#[test]
fn trips_flopped() {
    let combo: Vec<&Card> = vec![];
    // [6♣ 6♠], [9♦ 7❤ ] | A❤ 6❤ 9❤ | 4❤ | K♠	[6♣ 6♠] wins with Pair
    let holdings = [Card::from("6c").unwrap(), Card::from("6s").unwrap()];

    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("6h").unwrap(),
        Card::from("9h").unwrap(),
        Card::from("4h").unwrap(),
        Card::from("Ks").unwrap(),
    ];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    let holdings = [Card::from("9d").unwrap(), Card::from("7h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Trips(Rank::Six));
    assert_eq!(hand2.rank, HandRank::Flush(Suit::Hearts));
    assert_eq!(hand2 > hand1, true);
}

#[test]
fn straight_vs_straight() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Jh").unwrap(),
        Card::from("Ts").unwrap(),
        Card::from("9d").unwrap(),
        Card::from("3c").unwrap(),
        Card::from("2h").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    let holdings = [Card::from("7c").unwrap(), Card::from("8c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Straight(Rank::King));
    assert_eq!(hand2.rank, HandRank::Straight(Rank::Jack));
    assert_eq!(hand1 > hand2, true);
}

#[test]
fn straights() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Jh").unwrap(),
        Card::from("Ts").unwrap(),
        Card::from("9d").unwrap(),
        Card::from("3c").unwrap(),
        Card::from("2h").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("8h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Straight(Rank::Queen));

    let community_cards = [
        Card::from("8h").unwrap(),
        Card::from("Ts").unwrap(),
        Card::from("9d").unwrap(),
        Card::from("3c").unwrap(),
        Card::from("2h").unwrap(),
    ];

    let holdings = [Card::from("7c").unwrap(), Card::from("Jh").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Straight(Rank::Jack));

    // RoyalFlush: A♠ A♦ K♠ Q♠ J♠ T❤ 9♠
    let community_cards = [
        Card::from("Ad").unwrap(),
        Card::from("Qs").unwrap(),
        Card::from("Js").unwrap(),
        Card::from("Th").unwrap(),
        Card::from("9h").unwrap(),
    ];
    let holdings = [Card::from("As").unwrap(), Card::from("Ks").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_ne!(hand.rank, HandRank::RoyalFlush);
    assert_eq!(hand.rank, HandRank::Straight(Rank::Ace));
}

#[test]
fn straight_five_high() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("2s").unwrap(),
        Card::from("9d").unwrap(),
        Card::from("3c").unwrap(),
        Card::from("8h").unwrap(),
    ];

    let holdings = [Card::from("4c").unwrap(), Card::from("5h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Straight(Rank::Five));
}

#[test]
fn straight_and_flush() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Jh").unwrap(),
        Card::from("Tc").unwrap(),
        Card::from("9c").unwrap(),
        Card::from("3c").unwrap(),
        Card::from("2h").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::Flush(Suit::Clubs));
}

#[test]
fn full_house_on_board() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("As").unwrap(),
        Card::from("Kd").unwrap(),
        Card::from("Ac").unwrap(),
        Card::from("Kh").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("Jc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::FullHouse(Rank::Ace, Rank::King));
}

#[test]
fn full_house_flopped() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("As").unwrap(),
        Card::from("Kd").unwrap(),
        Card::from("7c").unwrap(),
        Card::from("Jh").unwrap(),
    ];

    let holdings = [Card::from("Ac").unwrap(), Card::from("Kc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::FullHouse(Rank::Ace, Rank::King));
}

#[test]
fn full_house_paired_board_on_river() {
    let combo: Vec<&Card> = vec![];
    // [T♦ 6♦], [A♦ 7♦] | Q♦ 8♣ A♠ | 7❤ | 7♠	[A♦ 7♦] wins with TwoPair
    let community_cards = [
        Card::from("Qd").unwrap(),
        Card::from("8c").unwrap(),
        Card::from("As").unwrap(),
        Card::from("7h").unwrap(),
        Card::from("7s").unwrap(),
    ];

    let holdings = [Card::from("Ad").unwrap(), Card::from("7d").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::FullHouse(Rank::Seven, Rank::Ace));
}

#[test]
fn full_house_paired_board_and_river() {
    let combo: Vec<&Card> = vec![];
    // [J❤ 5♠], [9♣ 6❤] | 2❤ 5♦ 2♠ | Q❤ | 5❤ 	[J❤ 5♠] wins with TwoPair
    let community_cards = [
        Card::from("2h").unwrap(),
        Card::from("5d").unwrap(),
        Card::from("2c").unwrap(),
        Card::from("Qh").unwrap(),
        Card::from("5h").unwrap(),
    ];

    let holdings = [Card::from("Jh").unwrap(), Card::from("5s").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::FullHouse(Rank::Five, Rank::Two));
}

#[test]
fn full_house_pockets_and_board_paired_on_river() {
    let combo: Vec<&Card> = vec![];
    // [K♣ 2♣], [8❤ 8♠] | 4❤ A♠ Q♣ | 8♦ | Q♦	[8❤ 8♠] wins with TwoPair
    let community_cards = [
        Card::from("4h").unwrap(),
        Card::from("As").unwrap(),
        Card::from("Qc").unwrap(),
        Card::from("8d").unwrap(),
        Card::from("Qd").unwrap(),
    ];

    let holdings = [Card::from("8h").unwrap(), Card::from("8s").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::FullHouse(Rank::Eight, Rank::Queen));
}

#[test]
fn quads_on_board() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("As").unwrap(),
        Card::from("Ad").unwrap(),
        Card::from("Ac").unwrap(),
        Card::from("Jh").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand1.rank, HandRank::Quads(Rank::Ace));

    let holdings = [Card::from("Tc").unwrap(), Card::from("9c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);
    assert_eq!(hand2.rank, HandRank::Quads(Rank::Ace));

    // better Kicker
    assert_eq!(hand1 > hand2, true);
}

#[test]
fn flush_vs_flush() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("6h").unwrap(),
        Card::from("9h").unwrap(),
        Card::from("4h").unwrap(),
        Card::from("Ks").unwrap(),
    ];

    let holdings = [Card::from("Jh").unwrap(), Card::from("7c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    let holdings = [Card::from("Th").unwrap(), Card::from("7c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Flush(Suit::Hearts));
    assert_eq!(hand2.rank, HandRank::Flush(Suit::Hearts));
    assert_eq!(hand1 > hand2, true);

    // [3♠ 2♣], [A♦ A♠] | 4♠ Q♠ J♠ | 7♣ | 9♠	¯\_(ツ)_/¯ Flush(Spades) vs. Flush(Spades)
    let community_cards = [
        Card::from("4s").unwrap(),
        Card::from("Qs").unwrap(),
        Card::from("Js").unwrap(),
        Card::from("7c").unwrap(),
        Card::from("9s").unwrap(),
    ];

    let holdings = [Card::from("3s").unwrap(), Card::from("2c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);

    let holdings = [Card::from("Ad").unwrap(), Card::from("As").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Flush(Suit::Spades));
    // assert_eq!(hand1.num_ranks, &[1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0]);
    assert_eq!(hand2.rank, HandRank::Flush(Suit::Spades));
    // assert_eq!(hand2.num_ranks, &[0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 2]);
    assert_eq!(hand1 < hand2, true);

    // [K♠ 2♣], [Q♠ A♣ ] | 4♠ J♠ A♠ | 7♣ | 9♠	¯\_(ツ)_/¯ Flush(Spades) vs. Flush(Spades)
    let community_cards = [
        Card::from("4s").unwrap(),
        Card::from("Js").unwrap(),
        Card::from("As").unwrap(),
        Card::from("7c").unwrap(),
        Card::from("9s").unwrap(),
    ];

    let holdings = [Card::from("Ks").unwrap(), Card::from("2c").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);
    let holdings = [Card::from("Qs").unwrap(), Card::from("Ac").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Flush(Suit::Spades));
    // assert_eq!(hero.num_ranks, &[1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
    assert_eq!(hand2.rank, HandRank::Flush(Suit::Spades));
    // assert_eq!(vilan.num_ranks, &[0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 2]);
    assert_eq!(hand1 > hand2, true);

    // [9♦ 7❤ ], [6♣ 6❤] | A❤ 8❤ 9❤ | 4❤ | K♠
    let community_cards = [
        Card::from("Ah").unwrap(),
        Card::from("8h").unwrap(),
        Card::from("9h").unwrap(),
        Card::from("4h").unwrap(),
        Card::from("Ks").unwrap(),
    ];

    let holdings = [Card::from("9d").unwrap(), Card::from("7h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand1 = Hand::new(&raw_cards, &combo);
    let holdings = [Card::from("6c").unwrap(), Card::from("6h").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand2 = Hand::new(&raw_cards, &combo);

    assert_eq!(hand1.rank, HandRank::Flush(Suit::Hearts));
    // assert_eq!(hero.num_ranks, &[0, 0, 1, 0, 0, 1, 1, 2, 0, 0, 0, 1, 1]);
    assert_eq!(hand2.rank, HandRank::Flush(Suit::Hearts));
    // assert_eq!(vilan.num_ranks, &[0, 0, 1, 0, 2, 0, 1, 1, 0, 0, 0, 1, 1]);
    assert_eq!(hand1 > hand2, true);
}

#[test]
fn straightflush() {
    let combo: Vec<&Card> = vec![];
    let community_cards = [
        Card::from("Jh").unwrap(),
        Card::from("Tc").unwrap(),
        Card::from("9c").unwrap(),
        Card::from("3c").unwrap(),
        Card::from("Jc").unwrap(),
    ];

    let holdings = [Card::from("Qc").unwrap(), Card::from("Kc").unwrap()];
    let raw_cards = RawData::from_chain(holdings.iter().chain(community_cards.iter()));
    let hand = Hand::new(&raw_cards, &combo);
    assert_eq!(hand.rank, HandRank::StraightFlush(Rank::King));
}
