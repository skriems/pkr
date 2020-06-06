pub mod card;
pub mod deck;
pub mod error;
pub mod holding;
pub mod prelude;
pub mod range;

/// A trait to determine wheter Self beats, splits or looses against another
pub trait Beats<Rhs: ?Sized = Self> {
    fn beats(&self, other: &Rhs) -> bool;

    fn splits(&self, other: &Rhs) -> bool;

    fn looses(&self, other: &Rhs) -> bool {
        !self.beats(other)
    }
}
