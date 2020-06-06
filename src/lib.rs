pub mod card;
pub mod deck;
pub mod error;
pub mod holding;
pub mod prelude;
pub mod range;


/// A trait to determine wheter Self beats the other 
pub trait Beat<Rhs: ?Sized = Self> {
    fn beats(&self, other: &Rhs) -> bool;
}
