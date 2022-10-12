#![doc = include_str!("../README.md")]

// Re-exports
pub use clap;
pub use clap_serde_proc::clap_serde;
pub use serde;

use serde::ser::Error;

/// Trait representing a struct which can be parsed from clap and serde.
/// This trait is automatically implemented by [`clap_serde`] procedural macro.
pub trait ClapSerde<'a>: Default + serde::Serialize {
    /// The same struct of the parent but with optional fields.
    type Opts: clap::Parser + serde::Deserialize<'a>;

    /// Merge in place from Opt struct
    fn update(&mut self, other: &mut Self::Opts);

    /// Merge from Opts struct
    fn merge(mut self, other: &mut Self::Opts) -> Self {
        self.update(other);
        self
    }

    /// Create new object from Opt
    fn from_opt(data: &mut Self::Opts) -> Self {
        Self::default().merge(data)
    }

    /// Merge from clap to the object
    fn merge_clap(self) -> Self {
        self.merge(&mut <Self::Opts as clap::Parser>::parse())
    }
    /// Create new object parsing from clap
    fn from_clap() -> Self {
        Self::default().merge_clap()
    }

    /// Merge parsed serde to the object
    fn merge_serde<E: Error>(self, parsed: Result<Self::Opts, E>) -> Result<Self, E> {
        Ok(self.merge(&mut parsed?))
    }
    /// Create new object from parsed serde
    fn from_serde<E: Error>(parsed: Result<Self::Opts, E>) -> Result<Self, E> {
        Self::default().merge_serde(parsed)
    }
}
