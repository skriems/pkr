use crate::card::*;

/// Meta Data to minimize processing of the internal `Board.ranks` and `Board.suits`. It's meant to
/// provide the basic _texture_ of the `Board` to the caller (mostly `HandResult`) for further
/// processing which needs to happen only once for N `Holdings`.
#[derive(Debug)]
pub struct BoardTexture {
    /// `true` as long as there no more than one `Suit` each
    pub is_rainbow: bool,
    /// has _one_ pair
    pub is_paired: bool,
    /// has _two_ pairs
    pub has_pairs: bool,
    /// has trips
    pub has_trips: bool,
    /// has a streight
    pub has_streight: bool,
    /// `true` if we have five cards of the same `Suit`
    pub has_flush: bool,
    /// `true` if we have at least two cards of same `Suit`
    pub flush_draw: bool,
    /// `true` if we have a least three cards of same `Suit`
    pub flush_with_suited: bool,
    /// `true` if we have a least four cards of same `Suit`
    pub flush_with_one: bool,
    /// `true` if we have quads on the board
    pub has_quads: bool,
}

/// The `Board` takes a slice of Cards and stores the count of their `Rank` and `Suit` as u8
/// arrays. The enums descriminators are used for indexing those arrays to infer the `Rank`.
///
/// Note that we do not skip a `Card` before dealing the Turn and River for easier `HandResult`
/// evaluation.  The caller would need to do that.
#[derive(Debug)]
pub struct Board<'a> {
    /// Slice of Cards
    cards: &'a [Card],
    /// Array of 13 usize for each respective `Rank`
    pub ranks: [usize; 13],
    /// Array of 4 usize for each respective `Suit`
    pub suits: [usize; 4],
    /// If the flop has been dealt
    pub flop_dealt: bool,
    /// If the turn card has been dealt
    pub turn_dealt: bool,
    /// If the river card has been dealt
    pub river_dealt: bool,
}

impl<'a> Board<'a> {
    /// create a new `Board` with `Cards`
    pub fn new(cards: &'a [Card]) -> Self {
        let ranks = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let suits = [0, 0, 0, 0];

        Board {
            cards,
            ranks,
            suits,
            flop_dealt: false,
            turn_dealt: false,
            river_dealt: false,
        }
    }

    /// Process the Flop
    pub fn with_flop(&mut self) -> Self {

        for card in &self.cards[..3] {
            self.ranks[card.rank as usize] += 1;
            self.suits[card.suit as usize] += 1;
        };

        Board { cards: self.cards,
            ranks: self.ranks,
            suits: self.suits,
            flop_dealt: true,
            turn_dealt: false,
            river_dealt: false,
        }
    }

    /// Process the Turn
    pub fn with_turn(&mut self) -> Self {
        let turn = &self.cards[3];
        self.ranks[turn.rank as usize] += 1;
        self.suits[turn.suit as usize] += 1;

        Board {
            cards: self.cards,
            ranks: self.ranks,
            suits: self.suits,
            flop_dealt: self.flop_dealt,
            turn_dealt: true,
            river_dealt: false,
        }
    }

    /// Process the River
    pub fn with_river(&mut self) -> Self {
        let river = &self.cards[4];
        self.ranks[river.rank as usize] += 1;
        self.suits[river.suit as usize] += 1;

        Board {
            cards: self.cards,
            ranks: self.ranks,
            suits: self.suits,
            flop_dealt: self.flop_dealt,
            turn_dealt: self.turn_dealt,
            river_dealt: true
        }
    }

    /// Process Flop, Turn and River
    pub fn full(&mut self) -> Self {
        self.with_flop().with_turn().with_river()
    }

    /// Return a slice of Cards
    pub fn cards(&self) -> &[Card] {
        if self.river_dealt {
            &self.cards[..5]
        } else if self.turn_dealt {
            &self.cards[..4]
        } else {
            &self.cards[..3]
        }
    }

    /// Return the flop slice
    pub fn flop(&self) -> &[Card] {
        &self.cards[..3]
    }

    /// Return the Turn `Card`
    pub fn turn(&self) -> &Card {
        &self.cards[3]
    }

    /// Return the River `Card`
    pub fn river(&self) -> &Card {
        &self.cards[4]
    }

    /// Return the Turn and River `Card` as slice, so we can use `.contains`
    pub fn turn_and_river(&self) -> &[Card] {
        &self.cards[3..4]
    }

    /// Determine the `Boards` HighCard
    pub fn high_card(&self) -> &Card {
        let mut high_card = &self.cards[0];
        if self.river_dealt {
            for card in &self.cards[..5] {
                if card >= high_card {
                    high_card = card;
                }
            }
        } else if self.turn_dealt {
            for card in &self.cards[..4] {
                if card >= high_card {
                    high_card = card;
                }
            }
        } else if self.flop_dealt {
            for card in &self.cards[..3] {
                if card >= high_card {
                    high_card = card;
                }
            }
        }
        high_card
    }

    /// Returns the `Rank` for a given discriminator of `Rank`
    pub fn get_rank(&self, discriminator: usize) -> Option<Rank> {
        for (idx, rank) in self.ranks.iter().enumerate() {
            if *rank == discriminator {
                return Some(Rank::from(idx));
            }
        }
        None
    }

    /// Returns a tuple of `Option<Rank>` for a paired board. Note that we might have two 
    pub fn pairs(&self) -> [Option<Rank>; 2] {
        let mut pairs = [None, None];
        for (idx, rank) in self.ranks.iter().enumerate() {
            if *rank == 2 && pairs[0].is_none() {
                 pairs[0] = Some(Rank::from(idx));
            } else if *rank == 2 {
                pairs[1] = Some(Rank::from(idx));
            }
        }
        pairs
    }

    /// Process the internal `ranks` and `suits` properties and return a `BoardTexture`
    pub fn texture(&self) -> BoardTexture {
        let mut is_rainbow = true;
        let mut is_paired = false;
        let mut has_pairs = false;
        let mut has_trips = false;
        let mut has_streight = false;
        let mut has_flush = false;
        let mut flush_draw = false;
        let mut flush_with_suited = false;
        let mut flush_with_one = false;
        let mut has_quads = false;

        for rank in self.ranks.iter() {
            match rank {
                2 => {
                    if is_paired {
                        has_pairs = true;
                    } else {
                        is_paired = true;
                    }
                }
                3 => has_trips = true,
                4 => has_quads = true,
                _ => continue
            }
        }

        for suit in self.suits.iter() {
            match suit {
                2 => {
                    flush_draw = true;
                    is_rainbow = false;
                }
                3 => {
                    flush_with_suited = true;
                    is_rainbow = false;
                }
                4 => {
                    flush_with_one = true;
                    is_rainbow = false;
                }
                5 => {
                    has_flush = true;
                    is_rainbow = false;
                }
                _ => continue
            }
        }
        BoardTexture {
            is_rainbow,
            is_paired,
            has_pairs,
            has_trips,
            has_streight,
            has_flush,
            flush_draw,
            flush_with_suited,
            flush_with_one,
            has_quads
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::*;

    #[test]
    fn ranks_basic() {
        let deck = Deck::default();
        let board = Board::new(&deck.cards).full();
        // the default deck is ordered so we expect
        // -  flop: A K Q
        // -  turn: J
        // - river: T
        assert_eq!(board.flop(), [Card::from("Ac").unwrap(),Card::from("Kc").unwrap(),Card::from("Qc").unwrap()]);
        assert_eq!(board.turn(), &Card::from("Jc").unwrap());
        assert_eq!(board.river(), &Card::from("Tc").unwrap());

        let expected_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        assert_eq!(board.ranks, expected_ranks);
        let expected_suits = [5, 0, 0, 0];
        assert_eq!(board.suits, expected_suits);

        assert_eq!(board.ranks[Rank::Ace as usize], 1);
        assert_eq!(board.ranks[Rank::King as usize], 1);
        assert_eq!(board.ranks[Rank::Queen as usize], 1);
        assert_eq!(board.ranks[Rank::Jack as usize], 1);
        assert_eq!(board.ranks[Rank::Ten as usize], 1);
    }

    #[test]
    fn texture_has_flush() {
        let deck = Deck::default();
        let board = Board::new(&deck.cards).full();
        let expected_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        assert_eq!(board.ranks, expected_ranks);
        let expected_suits = [5, 0, 0, 0];
        assert_eq!(board.suits, expected_suits);
        let texture = board.texture();
        assert_eq!(texture.has_flush, true);
    }

    #[test]
    fn high_card() {
        let deck = Deck::default();
        let board = Board::new(&deck.cards[1..4]).with_flop();
        println!("{:?}", board);
        println!("{:?}", board.high_card());
        assert_eq!(board.high_card() == &Card::from("Kc").unwrap(), true);

        let board_cards = [
            Card::from("Th").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("8s").unwrap(),
        ];
        let board = Board::new(&board_cards).with_flop();
        assert_eq!(board.high_card(), &Card::from("Th").unwrap());
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Board>(), 160);
    }
}
