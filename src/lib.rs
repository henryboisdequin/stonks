#![warn(missing_docs, rust_2018_idioms)]

//! Stock CLI tool called stonks which uses the Finnhub API.

/// CLI for Stonks.
pub mod cli;
/// Finnhub client for Stonks.
pub mod client;
/// Responsible for parsing toml files.
pub mod parse;
/// Util functions.
pub mod utils;
