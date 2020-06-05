use pkr::card::*;
use pkr::hand::*;
use pkr::holding::*;

fn main() {
    let first = Card::new(Rank::Ace, Suit::Clubs);
    let second = Card::new(Rank::Ace, Suit::Spades);
    println!("{:?}", first);
    let hand = Holding::new(first, second).unwrap();
    println!("{:?}", hand);
    let hand = Holding::from("AsKs").unwrap();
    println!("{:?}", hand);
}
