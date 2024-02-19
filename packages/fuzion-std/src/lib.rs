//! Interfaces for [Fuzions's](https://fuzion.app/flows) flows for vesting and streaming money

pub mod fees;

pub use fees::{FeeCollector, FeeReceiver, FeeResponse};
