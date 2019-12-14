//! shli provides a few raw building blocks for building your own shell-like CLI.
//! It uses termion and should thus be compatible with all terminals termion supports.
//!
//! An example:
//! ```no_run
//! extern crate shli;
//! use shli::split;
//! use shli::completion::Command;
//! use shli::Prompt;
//! 
//! fn main() {
//!     let mut p = Prompt::new("> ".to_string(), vec!(
//! 		Command::new("print").arg("--help"),
//! 		Command::new("echo"),
//! 		Command::new("exit")
//! 	));
//!     loop {
//!         match p.read_commandline() {
//!             Ok(line) => {
//!                 println!("");
//!                 if ! line.is_empty() {
//!                     match line[0].as_str() {
//!                         "exit" => break,
//!                         "print" | "echo" => if line.len() > 1 {
//!                             let mut output = line[1].clone();
//!                             for w in &line[2..] {
//!                                 output.push_str(&format!(" {}", w));
//!                             }
//!                             println!("{}", output);
//!                         }
//!                         cmd => println!("Did not find '{}' command!", cmd),
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

pub mod prompt;
pub mod parse;
pub mod completion;

pub use prompt::Prompt;
pub use parse::split;
pub use completion::Command;

#[cfg(test)]
mod tests;
