use crate::parse::split;
use std::io::{Error, ErrorKind, Write};
use termion::cursor;
use termion::event::Key::{self, Alt, Char, Ctrl};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use std::io::{stdin, stdout};

fn reprint(
	stdout: &mut RawTerminal<std::io::StdoutLock>,
	prompt: &str,
	line: &String,
	right_line: &String,
) -> std::io::Result<()> {
	write!(stdout, "\r{}{}{}", prompt, line, right_line)?;
	if right_line.len() != 0 {
		write!(stdout, "{}", cursor::Left(right_line.len() as u16))?;
	}
	stdout.flush()?;
	Ok(())
}

/// Config struct for building command line interfaces.
/// An example:
/// ```
/// use shli::{Prompt,prefix_completion,split};
/// 
/// fn example_completion(line: &str) -> Vec<String> {
///     let cmd = split(&line);
///     if cmd.len() == 1 {
///         prefix_completion(&cmd[0], &["print", "echo", "exit"])
///     } else if cmd.len() == 0 {
///         prefix_completion("", &["print", "echo", "exit"])
///     } else {
///         vec!()
///     }
/// }
/// 
/// let p = Prompt::new("> ".to_string(), example_completion);
/// ```
/// Now, use the `read_commandline` method to let the user type a command.
pub struct Prompt {
	pub prompt_text: String,
	pub completion: Box<dyn Fn(&str) -> Vec<String>>,
	pub history: Vec<String>,
}

impl Prompt {
	pub fn new(prompt_text: String, completion: impl Fn(&str) -> Vec<String> + 'static,) -> Prompt {
		Prompt {
			prompt_text: prompt_text,
			completion: Box::new(completion),
			history: vec!(),
		}
	}

	/// Prompt for a single command line.
	///
	/// This function reads and returns a command line.
	/// Line editing with backspace and ALT+backspace is supported.
	///
	/// If TAB is pressed by the user, the callback function `completion` is asked
	/// for possible argument completion. If it returns exactly 1 completion, it
	/// is used, if it returns more, they are displayed.
	///
	/// If Ctrl+C is pressed, this function returns `Err(Error::new(ErrorKind::Other, "Ctrl-C pressed.")`,
	/// while an EOF of `stdin` or Ctrl+D will return the error type `ErrorKind::UnexpectedEof`.
	///
	/// On success, `read_commandline` returns a Vector of command line components (command + arguments).
	///
	/// For example, the following input:
	/// ```text
	/// > print out "example command"
	/// ```
	/// will return `vec!["print", "out", "example command"]`.
	pub fn read_commandline(&mut self) -> std::io::Result<Vec<String>> {
		let stdout = stdout();
		let mut stdout = stdout.lock().into_raw_mode()?;
		let stdin = stdin();
		let stdin = stdin.lock();
		write!(stdout, "{}", &self.prompt_text)?;
		stdout.flush()?;
		let mut line = String::new();
		let mut right_line = String::new();
		let mut history_offset = 0;
	
		for key in stdin.keys() {
			match key {
				Ok(Char('\n')) => break,
				Ok(Char('\t')) => {
					// The tabulator was pressed.
					let possible_words = (self.completion)(&line);
					if possible_words.len() == 0 {
					} else if possible_words.len() == 1 {
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
						reprint(&mut stdout, &self.prompt_text, &line, &right_line)?;
					} else {
						// Display the possibilities
						write!(
							stdout,
							"\n\r Completions: {:?}\n\r> {}",
							possible_words, line
						)?;
						stdout.flush()?;
					}
				}
				Ok(Char(ch)) => {
					line.push(ch);
					reprint(&mut stdout, &self.prompt_text, &line, &right_line)?;
				}
				Ok(Key::Left) => {
					if let Some(ch) = line.pop() {
						right_line = format!("{}{}", ch, right_line);
						write!(stdout, "{}", cursor::Left(1))?;
						stdout.flush()?;
					}
				}
				Ok(Key::Right) => {
					if right_line.len() > 0 {
						line.push(right_line.remove(0));
						write!(stdout, "{}", cursor::Right(1))?;
						stdout.flush()?;
					}
				}
				Ok(Key::Up) => {
					if history_offset < self.history.len() {
						history_offset += 1;
						if let Some(new_cmd_line) = self.history.get(self.history.len() - history_offset) {
							let chars_to_wipe = self.prompt_text.len() + line.len() + right_line.len();
							line = new_cmd_line.clone();
							write!(stdout, "\r")?;
							for _ in 0..chars_to_wipe {
								write!(stdout, " ")?;
							}
							right_line = String::new();
							reprint(&mut stdout, &self.prompt_text, &line, &right_line)?
						}
					}
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
					// ALT+TAB was pressed.
					// Remove the last word.
					let mut words = split(&line);
					if let Some(_) = words.pop() {
						let old_len = line.len();
						// Build up the cmdline again
						line = String::new();
						for word in words {
							line.push_str(&word);
							line.push(' ');
						}
						// Wipe removed characters
						if line.len() < old_len {
							write!(
								stdout,
								"{}{}",
								cursor::Left((old_len - line.len()) as u16),
								" ".repeat(old_len)
							)?;
						}
						// Now display the new cmdline
						reprint(&mut stdout, &self.prompt_text, &line, &right_line)?;
					}
				}
				Ok(_) => {}
				Err(e) => return Err(e),
			}
		}
		line.push_str(&right_line);
		self.history.push(line.clone());
		Ok(split(&line))
	}
	
}