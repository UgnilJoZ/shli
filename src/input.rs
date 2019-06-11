use std::io::{Stdout, Write, Error, ErrorKind};
use termion::event::Key::{self, Char, Ctrl, Alt};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use crate::parse::split;

/// Prompt for a single command line.
/// 
/// This function reads and returns a command line, after prompting with `> `.
/// Line editing with backspace and ALT+backspace is supported. If TAB is pressed, the
/// callback function `completion` is asked for possible argument completion. If it returns
/// exactly 1 completion, it is used, if it returns more, they are displayed.
/// 
/// If Ctrl+C is pressed, this function returns `Err(Error::new(ErrorKind::Other, "Ctrl-C pressed.")`,
/// while an EOF of `stdin` or Ctrl+D will return the error type `ErrorKind::UnexpectedEof`.
/// 
/// On success, `read_commandline` returns a Vector of command line components (command + arguments).
/// 
/// An example:
/// ```rust,no_run
/// use std::io::{stdin, stdout};
/// extern crate shli;
/// use shli::split;
/// use shli::read_commandline;
/// 
/// fn example_completion(line: &str) -> Vec<String> {
///     let cmd = split(&line);
///     if cmd.len() == 1 {
///         ["Hallo", "Tschüs", "exit"]
///         .iter()
///         .filter(|&e| {
///             (*e).starts_with(&cmd[0])
///             })
///         .map(|s| s.to_string())
///         .collect()
///     } else if cmd.len() == 0 {
///         vec!("Hallo".to_string(), "Tschüs".to_string(), "exit".to_string())
///     } else {
///         vec!()
///     }
/// }
/// 
/// fn main() {
///     loop {
///         let stdin = stdin();
///         let line_result = read_commandline(stdin.lock(), &mut stdout(), example_completion);
///         match line_result {
///             Ok(line) => {
///                 println!("");
///                 if ! line.is_empty() {
///                     match line[0].as_str() {
///                         "exit" => break,
///                         cmd => println!("I din't find {}!", cmd),
///                     }
///                 }
///             }
///             Err(e) => {
///                 match e.kind() {
///                     std::io::ErrorKind::UnexpectedEof => {
///                         println!("exit");
///                         break;
///                     }
///                     std::io::ErrorKind::Other => {
///                         println!("\nCtrl+C pressed.");
///                     }
///                     _ => {
///                         println!("Reading error: {:?}", e);
///                     }
///                 };
///             }
///         }
///     }
/// }
/// ```
pub fn read_commandline(stdin: std::io::StdinLock<>, stdout: &mut Stdout, completion: impl Fn(&str) -> Vec<String>) -> std::io::Result<Vec<String>> {
    let mut stdout = stdout.into_raw_mode().unwrap();
    write!(stdout, "> ")?;
    stdout.flush()?;
    let mut line = String::new();

    for key in stdin.keys() {
        match key {
            Ok(Char('\n')) => break,
            Ok(Char('\t')) => {
                let possible_words = completion(&line);
                if possible_words.len() == 0 { }
                else if possible_words.len() == 1 {
                    // First, replace the last word
                    let mut words = split(&line);
                    words.pop();
                    words.push(possible_words[0].clone());
                    // Now build up the cmdline again
					line = String::new();
                    for word in words {
                        line.push_str(&word);
						line.push(' ');
                    }
                    // Now display the new cmdline
                    write!(stdout, "\r> {}", line)?;
                    stdout.flush()?;
                } else {
                    // Display the possibilities
                    write!(stdout, "\n\r Completions: {:?}\n\r> {}", possible_words, line)?;
                    stdout.flush()?;
                }
            }
            Ok(Char(ch)) => {
                write!(stdout, "{}", ch)?;
                stdout.flush()?;
                line.push(ch);
            }
            Ok(Ctrl('c')) => {
                return Err(Error::new(ErrorKind::Other, "Ctrl-C pressed."));
            }
             Ok(Ctrl('d')) => {
                return Err(Error::new(ErrorKind::UnexpectedEof, ""));
            }
            Ok(Key::Backspace) => {
                if line.pop().is_some() {
                    write!(stdout, "{} {}", cursor::Left(1), cursor::Left(1))?;
                    stdout.flush()?;
                }
            }
            Ok(Alt('\u{7f}')) => {
				// First, remove the last word
                let mut words = split(&line);
                while let Some(_) = words.pop() {
					if line.pop().is_some() {
						write!(stdout, "{} {}", cursor::Left(1), cursor::Left(1))?;
					}
				}
				stdout.flush()?;
            }
            Ok(_) => {} //{println!("{:?}", sonst)}
            Err(e) => {return Err(e)}
        }
    }
    Ok(split(&line))
}
