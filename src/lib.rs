// Copyright (C) 2022 Davide Peressoni
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![doc(
    html_favicon_url = "https://gitlab.com/DPDmancul/clap-serde-derive/-/raw/main/asstets/logo.svg"
)]
#![doc(html_logo_url = "https://gitlab.com/DPDmancul/clap-serde-derive/-/raw/main/assets/logo.svg")]
#![doc = include_str!("../README.md")]
#![no_std]

use core::borrow::BorrowMut;

// Re-exports
pub use clap;
pub use clap_serde_proc::ClapSerde;
pub use serde;

/// Trait representing a struct which can be parsed from clap and serde.
/// This trait is automatically implemented by [`ClapSerde`][clap_serde_proc::ClapSerde]
/// derive macro.
pub trait ClapSerde: Default + From<Self::Opt> + for<'a> From<&'a mut Self::Opt> {
    /// The same struct of the parent but with optional fields.
    type Opt: Default + clap::Parser + serde::de::DeserializeOwned + clap::Args;

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
