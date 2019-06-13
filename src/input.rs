use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use crate::parse::split;
use std::io::{Error, ErrorKind, Stdout, Write};
use termion::cursor;
use termion::event::Key::{self, Alt, Char, Ctrl};
use termion::input::TermRead;

/// Prompt for a single command line.
///
/// This function reads and returns a command line, after prompting with `> `.
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
pub fn read_commandline(
    stdin: std::io::StdinLock,
    stdout: &mut Stdout,
    completion: impl Fn(&str) -> Vec<String>,
) -> std::io::Result<Vec<String>> {
    let mut stdout = stdout.into_raw_mode()?;
    write!(stdout, "> ")?;
    stdout.flush()?;
    let mut line = String::new();
    let mut right_line = String::new();

    fn reprint(stdout: &mut RawTerminal<&mut std::io::Stdout>, line: &String, right_line: &String) -> std::io::Result<()> {
        write!(stdout, "\r> {}{}", line, right_line)?;
        if right_line.len() != 0 {
            write!(stdout, "{}", cursor::Left(right_line.len() as u16))?;
        }
        stdout.flush()?;
        Ok(())
    };

    for key in stdin.keys() {
        match key {
            Ok(Char('\n')) => break,
            Ok(Char('\t')) => {
                // The tabulator was pressed.
                let possible_words = completion(&line);
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
                    reprint(&mut stdout, &line, &right_line)?;
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
                reprint(&mut stdout, &line, &right_line)?;
            }
            Ok(Key::Left) => if let Some(ch) = line.pop() {
                right_line = format!("{}{}", ch, right_line);
                write!(stdout, "{}", cursor::Left(1))?;
                stdout.flush()?;
            }
            Ok(Key::Right) => if right_line.len() > 0 {
                line.push(right_line.remove(0));
                write!(stdout, "{}", cursor::Right(1))?;
                stdout.flush()?;
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
                if let Some(w) = words.pop() {
                    let old_len = line.len();
                    // Build up the cmdline again
                    line = String::new();
                    for word in words {
                        line.push_str(&word);
                        line.push(' ');
                    }
                    // Wipe removed characters
                    if line.len() < old_len {
                        write!(stdout, "{}{}", cursor::Left((old_len - line.len()) as u16), " ".repeat(old_len))?;
                    }
                    // Now display the new cmdline
                    reprint(&mut stdout, &line, &right_line)?;
                }
            }
            Ok(_) => {} //{println!("{:?}", sonst)}
            Err(e) => return Err(e),
        }
    }
    line.push_str(&right_line);
    Ok(split(&line))
}
