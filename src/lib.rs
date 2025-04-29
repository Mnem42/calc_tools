//! A simple calculator system that attempts to be as modular as possible.
//! 
#![warn(missing_docs)]

/// SI units
pub mod si;

/// General things that don't really fit anywhere else
pub mod r#impl;

/// Return types for calculators
pub mod calc_return;

/// TUI runner
pub mod tui_runner;

/// Demo calculator
#[cfg(feature = "demo-calc")]
pub mod demo;
