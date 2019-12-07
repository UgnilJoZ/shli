//! shli provides a few raw building blocks for building your own shell-like CLI.
//! It uses termion and should thus be compatible with all terminals termion supports.
//!
//! An example:
//! ```no_run
//! use std::io::{stdin, stdout};
//! extern crate shli;
//! use shli::split;
//! use shli::read_commandline;
//! use shli::parse::prefix_completion;
//!
//! fn example_completion(line: &str) -> Vec<String> {
//!     let cmd = split(&line);
//!     if cmd.len() == 1 {
//!         prefix_completion(&cmd[0], &["Hallo", "Tschüs", "exit"])
//!     } else if cmd.len() == 0 {
//!         prefix_completion("", &["Hallo", "Tschüs", "exit"])
//!     } else {
//!         vec!()
//!     }
//! }
//!
//! fn main() {
//!     loop {
//!         let stdin = stdin();
//!         let line_result = read_commandline(stdin.lock(), &mut stdout(), example_completion);
//!         match line_result {
//!             Ok(line) => {
//!                 println!("");
//!                 if ! line.is_empty() {
//!                     match line[0].as_str() {
//!                         "exit" => break,
//!                         cmd => println!("I din't find {}!", cmd),
//!                     }
//!                 }
//!             }
//!             Err(e) => {
//!                 match e.kind() {
//!                     std::io::ErrorKind::UnexpectedEof => {
//!                         println!("exit");
//!                         break;
//!                     }
//!                     std::io::ErrorKind::Other => {
//!                         println!("\nCtrl+C pressed.");
//!                     }
//!                     _ => {
//!                         println!("Reading error: {:?}", e);
//!                     }
//!                 };
//!             }
//!         }
//!     }
//! }
//! ```

extern crate termion;

pub mod input;
pub mod parse;

pub use input::read_commandline;
pub use parse::split;

#[cfg(test)]
mod tests;
