//! Analysis of Workflow Description Language (WDL) documents.
//!
//! An analyzer can be used to implement the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/).
#![doc = include_str!("../RULES.md")]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::broken_intra_doc_links)]

mod analyzer;
pub mod diagnostics;
pub mod document;
pub mod eval;
mod graph;
mod queue;
mod rayon;
mod rules;
pub mod stdlib;
pub mod types;

pub use analyzer::*;
pub use rules::*;
