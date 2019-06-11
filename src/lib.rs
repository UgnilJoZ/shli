//! shli provides a few raw building blocks for building your own shell-like CLI.
//! It uses termion and should thus be compatible with all terminals termion supports.
//! 
//! See `read_commandline` for an example with a bit tab completion.

extern crate termion;

pub mod parse;
pub mod input;

pub use input::read_commandline;
pub use parse::split;

#[cfg(test)]
mod tests;