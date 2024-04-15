//! Contains manipulations of address and address patterns.

mod core;

#[cfg(feature = "address-pattern")]
mod pattern;

pub use self::core::Address;

#[cfg(feature = "address-pattern")]
pub use self::pattern::{AddressPattern, Expression};
