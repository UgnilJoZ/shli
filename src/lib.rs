//! shli provides a few raw building blocks for building your own shell-like CLI.
//! It uses termion and should thus be compatible with all terminals termion supports.
//!
//! An example:
//! ```no_run
//! extern crate shli;
//! use shli::split;
//! use shli::parse::prefix_completion;
//! use shli::Prompt;
//! 
//! fn example_completion(line: &str) -> Vec<String> {
//!     let cmd = split(&line);
//!     if cmd.len() == 1 {
//!         prefix_completion(&cmd[0], &["print", "echo", "exit"])
//!     } else if cmd.len() == 0 {
//!         prefix_completion("", &["print", "echo", "exit"])
//!     } else {
//!         vec!()
//!     }
//! }
//! 
//! fn main() {
//!     let mut p = Prompt::new("> ".to_string(), example_completion);
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
pub use parse::prefix_completion;

#[cfg(test)]
mod tests;
