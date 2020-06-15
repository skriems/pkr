use crate::card::*;

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
    pub ranks: [[usize; 4]; 13],
    pub num_ranks: [usize; 13],
    pub num_suits: [usize; 4],
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
        let ranks = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];

        let num_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let num_suits = [0, 0, 0, 0];

        Board {
            cards,
            ranks,
            num_ranks,
            num_suits,
            flop_dealt: false,
            turn_dealt: false,
            river_dealt: false,
        }
    }

    /// Process the Flop
    pub fn with_flop(&mut self) -> Self {
        for card in &self.cards[..3] {
            let rank = card.rank as usize;
            let suit = card.suit as usize;
            self.ranks[rank][suit] = 1;
            self.num_ranks[rank] += 1;
            self.num_suits[suit] += 1;
        }

        Board {
            cards: self.cards,
            ranks: self.ranks,
            num_ranks: self.num_ranks,
            num_suits: self.num_suits,
            flop_dealt: true,
            turn_dealt: false,
            river_dealt: false,
        }
    }

    /// Process the Turn
    pub fn with_turn(&mut self) -> Self {
        let turn = &self.cards[3];
        let rank = turn.rank as usize;
        let suit = turn.suit as usize;
        self.ranks[rank][suit] = 1;
        self.num_ranks[rank] += 1;
        self.num_suits[suit] += 1;

        Board {
            cards: self.cards,
            ranks: self.ranks,
            num_ranks: self.num_ranks,
            num_suits: self.num_suits,
            flop_dealt: self.flop_dealt,
            turn_dealt: true,
            river_dealt: false,
        }
    }

    /// Process the River
    pub fn with_river(&mut self) -> Self {
        let river = &self.cards[4];
        let rank = river.rank as usize;
        let suit = river.suit as usize;
        self.ranks[rank][suit] = 1;
        self.num_ranks[rank] += 1;
        self.num_suits[suit] += 1;

        Board {
            cards: self.cards,
            ranks: self.ranks,
            num_ranks: self.num_ranks,
            num_suits: self.num_suits,
            flop_dealt: self.flop_dealt,
            turn_dealt: self.turn_dealt,
            river_dealt: true,
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

    /// Returns a tuple of `Option<Rank>` for a paired board. Note that we might have two
    pub fn pairs(&self) -> [Option<Rank>; 2] {
        let mut pairs = [None, None];
        for (idx, rank) in self.num_ranks.iter().enumerate() {
            if *rank == 2 && pairs[0].is_none() {
                pairs[0] = Some(Rank::from(idx));
            } else if *rank == 2 {
                pairs[1] = Some(Rank::from(idx));
            }
        }
        pairs
    }

    pub fn texture(&self) -> BoardTexture {
        BoardTexture::new(&self)
    }
}

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
    /// straight high card
    pub straight_high: Option<Rank>,
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

impl BoardTexture {
    /// pre-process a `Board` and return its `BoardTexture`. This improves evaluation speed for N
    /// `Holdings`
    pub fn new(board: &Board) -> Self {
        let mut is_rainbow = true;
        let mut is_paired = false;
        let mut has_pairs = false;
        let mut has_trips = false;
        let mut has_flush = false;
        let mut flush_draw = false;
        let mut flush_with_suited = false;
        let mut flush_with_one = false;
        let mut has_quads = false;

        let mut last_idx = 0;
        let mut connected = 0;

        for (idx, amount) in board.num_ranks.iter().enumerate() {
            match amount {
                1 => {
                    if idx > 0 && last_idx != idx - 1 {
                        connected = 0;
                    }
                    connected += 1;
                    last_idx = idx;
                }
                2 => {
                    if is_paired {
                        has_pairs = true;
                    } else {
                        is_paired = true;
                    }
                }
                3 => has_trips = true,
                4 => has_quads = true,
                _ => continue,
            }
        }

        let mut straight_high = None;
        if connected == 5 {
            straight_high = Some(Rank::from(last_idx));
        }

        for suit in board.num_suits.iter() {
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
                _ => continue,
            }
        }

        BoardTexture {
            is_rainbow,
            is_paired,
            has_pairs,
            has_trips,
            straight_high,
            has_flush,
            flush_draw,
            flush_with_suited,
            flush_with_one,
            has_quads,
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
        assert_eq!(
            board.flop(),
            [
                Card::from("Ac").unwrap(),
                Card::from("Kc").unwrap(),
                Card::from("Qc").unwrap()
            ]
        );
        assert_eq!(board.turn(), &Card::from("Jc").unwrap());
        assert_eq!(board.river(), &Card::from("Tc").unwrap());

        let expected_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        assert_eq!(board.num_ranks, expected_ranks);
        let expected_suits = [5, 0, 0, 0];
        assert_eq!(board.num_suits, expected_suits);

        assert_eq!(board.num_ranks[Rank::Ace as usize], 1);
        assert_eq!(board.num_ranks[Rank::King as usize], 1);
        assert_eq!(board.num_ranks[Rank::Queen as usize], 1);
        assert_eq!(board.num_ranks[Rank::Jack as usize], 1);
        assert_eq!(board.num_ranks[Rank::Ten as usize], 1);
    }

    #[test]
    fn texture_has_flush() {
        let deck = Deck::default();
        let board = Board::new(&deck.cards).full();
        let expected_ranks = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        assert_eq!(board.num_ranks, expected_ranks);
        let expected_suits = [5, 0, 0, 0];
        assert_eq!(board.num_suits, expected_suits);
        let texture = board.texture();
        assert_eq!(texture.has_flush, true);
    }

    #[test]
    fn texture_has_straight() {
        let board_cards = [
            Card::from("Jd").unwrap(),
            Card::from("Ts").unwrap(),
            Card::from("9d").unwrap(),
            Card::from("8c").unwrap(),
            Card::from("7h").unwrap(),
        ];
        let board = Board::new(&board_cards).full();
        let texture = board.texture();
        assert_eq!(texture.straight_high, Some(Rank::Jack));
    }

    #[test]
    fn mem() {
        assert_eq!(std::mem::size_of::<Board>(), 160);
        assert_eq!(std::mem::size_of::<BoardTexture>(), 10);
    }
}
