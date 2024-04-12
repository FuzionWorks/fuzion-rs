//! Interfaces for [Fuzions's](https://fuzion.app/flows) flows for vesting and streaming money

pub mod execute;
pub mod instantiate;
pub mod query;

pub use {execute::*, instantiate::*, query::*};
