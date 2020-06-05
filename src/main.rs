use pkr::card::*;
use pkr::hand::*;

fn main() {
    let first = Card::new(Rank::Ace, Suit::Clubs);
    let second = Card::new(Rank::Ace, Suit::Spades);
    println!("{:?}", first);
    let hand = Hand::new(first, second).unwrap();
    println!("{:?}", hand);
    let hand = Hand::from("AsKs").unwrap();
    println!("{:?}", hand);
}
