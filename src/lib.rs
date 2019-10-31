//! A crate containing general purpose iterators not included in the standard library but quite useful. Regrouped in this crate by usage

pub mod filter;

pub mod share;

mod core;
pub use crate::core::*;

pub mod child;
