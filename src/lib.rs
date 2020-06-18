pub mod card;
pub mod error;
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
