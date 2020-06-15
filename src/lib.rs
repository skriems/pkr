//! TODO: elaborate
//! - Deck
//!     - Hand
//!         - Holding
//!         - Board
//!             - BoardTexture
//!     - HandResult (Holding, Board, BoardTexture)
//!         - HandRank

pub mod board;
pub mod card;
pub mod deck;
pub mod error;
pub mod hand;
pub mod holding;
pub mod prelude;
pub mod range;
pub mod result;

/// A trait to determine wheter Self beats, splits or looses against another
pub trait Beats<Rhs: ?Sized = Self> {
    fn beats(&self, other: &Rhs) -> bool;

    fn pairs(&self, other: &Rhs) -> bool;

    fn looses(&self, other: &Rhs) -> bool {
        !self.beats(other) && !self.pairs(other)
    }
}
