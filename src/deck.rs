use crate::card::{Card, Rank, Suit};
use crate::error::{Error, Result};
use crate::prelude::StringChunks;
use std::collections::HashSet;

pub fn get_cards(args: &[String]) -> Result<(Vec<Card>, Vec<Card>, Vec<Card>, HashSet<Card>)> {
    let mut deck: HashSet<Card> = HashSet::new();
    deck.insert(Card::new(Rank::Ace, Suit::Clubs));
    deck.insert(Card::new(Rank::King, Suit::Clubs));
    deck.insert(Card::new(Rank::Queen, Suit::Clubs));
    deck.insert(Card::new(Rank::Jack, Suit::Clubs));
    deck.insert(Card::new(Rank::Ten, Suit::Clubs));
    deck.insert(Card::new(Rank::Nine, Suit::Clubs));
    deck.insert(Card::new(Rank::Eight, Suit::Clubs));
    deck.insert(Card::new(Rank::Seven, Suit::Clubs));
    deck.insert(Card::new(Rank::Six, Suit::Clubs));
    deck.insert(Card::new(Rank::Five, Suit::Clubs));
    deck.insert(Card::new(Rank::Four, Suit::Clubs));
    deck.insert(Card::new(Rank::Three, Suit::Clubs));
    deck.insert(Card::new(Rank::Two, Suit::Clubs));
    deck.insert(Card::new(Rank::Ace, Suit::Spades));
    deck.insert(Card::new(Rank::King, Suit::Spades));
    deck.insert(Card::new(Rank::Queen, Suit::Spades));
    deck.insert(Card::new(Rank::Jack, Suit::Spades));
    deck.insert(Card::new(Rank::Ten, Suit::Spades));
    deck.insert(Card::new(Rank::Nine, Suit::Spades));
    deck.insert(Card::new(Rank::Eight, Suit::Spades));
    deck.insert(Card::new(Rank::Seven, Suit::Spades));
    deck.insert(Card::new(Rank::Six, Suit::Spades));
    deck.insert(Card::new(Rank::Five, Suit::Spades));
    deck.insert(Card::new(Rank::Four, Suit::Spades));
    deck.insert(Card::new(Rank::Three, Suit::Spades));
    deck.insert(Card::new(Rank::Two, Suit::Spades));
    deck.insert(Card::new(Rank::Ace, Suit::Hearts));
    deck.insert(Card::new(Rank::King, Suit::Hearts));
    deck.insert(Card::new(Rank::Queen, Suit::Hearts));
    deck.insert(Card::new(Rank::Jack, Suit::Hearts));
    deck.insert(Card::new(Rank::Ten, Suit::Hearts));
    deck.insert(Card::new(Rank::Nine, Suit::Hearts));
    deck.insert(Card::new(Rank::Eight, Suit::Hearts));
    deck.insert(Card::new(Rank::Seven, Suit::Hearts));
    deck.insert(Card::new(Rank::Six, Suit::Hearts));
    deck.insert(Card::new(Rank::Five, Suit::Hearts));
    deck.insert(Card::new(Rank::Four, Suit::Hearts));
    deck.insert(Card::new(Rank::Three, Suit::Hearts));
    deck.insert(Card::new(Rank::Two, Suit::Hearts));
    deck.insert(Card::new(Rank::Ace, Suit::Diamonds));
    deck.insert(Card::new(Rank::King, Suit::Diamonds));
    deck.insert(Card::new(Rank::Queen, Suit::Diamonds));
    deck.insert(Card::new(Rank::Jack, Suit::Diamonds));
    deck.insert(Card::new(Rank::Ten, Suit::Diamonds));
    deck.insert(Card::new(Rank::Nine, Suit::Diamonds));
    deck.insert(Card::new(Rank::Eight, Suit::Diamonds));
    deck.insert(Card::new(Rank::Seven, Suit::Diamonds));
    deck.insert(Card::new(Rank::Six, Suit::Diamonds));
    deck.insert(Card::new(Rank::Five, Suit::Diamonds));
    deck.insert(Card::new(Rank::Four, Suit::Diamonds));
    deck.insert(Card::new(Rank::Three, Suit::Diamonds));
    deck.insert(Card::new(Rank::Two, Suit::Diamonds));

    let mut hero: Vec<Card> = Vec::with_capacity(2); // 2 holdings Cards
    let mut vilan: Vec<Card> = Vec::with_capacity(2); // 2 holdings Cards
    let mut community_cards: Vec<Card> = Vec::with_capacity(5); // up to 5 community_cards

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        for card_string in StringChunks::new(&arg, 2) {
            if let Ok(card) = Card::from(card_string) {
                if i == 1 {
                    hero.push(deck.take(&card).ok_or(Error::DuplicateCard)?);
                } else if i == 2 {
                    vilan.push(deck.take(&card).ok_or(Error::DuplicateCard)?);
                } else {
                    community_cards.push(deck.take(&card).ok_or(Error::DuplicateCard)?);
                }
            }
        }
    }
    Ok((hero, vilan, community_cards, deck))
}

#[derive(Debug)]
pub struct Deck {
    pub cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Self {
        let cards: [Card; 52] = [
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
        ];
        Deck { cards }
    }
}
