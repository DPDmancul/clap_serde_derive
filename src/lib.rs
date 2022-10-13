#![doc = include_str!("../README.md")]

use std::borrow::BorrowMut;

// Re-exports
pub use clap;
pub use clap_serde_proc::clap_serde;
pub use serde;

/// Trait representing a struct which can be parsed from clap and serde.
/// This trait is automatically implemented by [`clap_serde`] procedural macro.
pub trait ClapSerde:
    Default + serde::Serialize + From<Self::Opt> + for<'a> From<&'a mut Self::Opt>
{
    /// The same struct of the parent but with optional fields.
    type Opt: clap::Parser + serde::de::DeserializeOwned;

    /// Merge in place from Opt struct.
    ///
    /// Fields which are not None in `other` will be cleared and used to update `self`.
    /// Fields which are None in `other` will not be modified in `self`.
    fn update(&mut self, other: impl BorrowMut<Self::Opt>);

    /// Merge from Opts struct.
    fn merge(mut self, other: impl BorrowMut<Self::Opt>) -> Self {
        self.update(other);
        self
    }

    /// Merge from clap to the object.
    fn merge_clap(self) -> Self {
        self.merge(<Self::Opt as clap::Parser>::parse())
    }
    /// Create new object parsing from clap.
    fn from_clap() -> Self {
        Self::default().merge_clap()
    }
}
