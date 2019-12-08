/// Simple state machine for processing escaping within command line strings.
///
/// An example usage:
/// ```
/// use shli::parse::EscapingState;
///
/// // This will build the string `some_command "some string\"" "some other string`
/// let cmdline = "some_command \"some string\\\"\" \"some other string";
/// let mut state = EscapingState::process(cmdline);
///
/// // A finished command line string should not pass this assertion.
/// // This will succeed, because we made an error at `"some other string`.
/// // A following whitespace would be escaped.
/// assert!(state.whitespace_escaped())
/// ```
#[derive(Debug)]
pub struct EscapingState {
    /// A single quote is active.
    pub single_quote: bool,
    /// A double quote is active.
    pub double_quote: bool,
    /// A backslash is active.
    pub backslash: bool,
}

impl EscapingState {
    pub fn new() -> EscapingState {
        EscapingState {
            single_quote: false,
            double_quote: false,
            backslash: false,
        }
    }

    /// Call this function to proceed on the input string.
    pub fn step(&mut self, ch: char) {
        match ch {
            '"' => {
                if !self.doublequote_escaped() {
                    self.double_quote = !self.double_quote
                }
            }
            '\'' => {
                if !self.singlequote_escaped() {
                    self.single_quote = !self.single_quote
                }
            }
            _ => {}
        }

        if self.backslash {
            self.backslash = false
        } else if ch == '\\' {
            self.backslash = true
        }
    }

    /// If the next character would be whitespace, would it be escaped
    /// or viewed as a component delimiter?
    pub fn whitespace_escaped(&self) -> bool {
        self.single_quote || self.double_quote || self.backslash
    }

    /// If the next character would be `"`, would it be escaped
    /// or would it start/end a string sequence, in which whitespace and single
    /// quote are escaped? (`"A B C \""`)
    pub fn doublequote_escaped(&self) -> bool {
        self.single_quote || self.backslash
    }

    /// If the next character would be `'`, would it be escaped
    /// or would it start/end a string sequence, in which whitespace and double
    /// quotes are escaped? (`'A B C \''`)
    pub fn singlequote_escaped(&self) -> bool {
        self.double_quote || self.backslash
    }

    /// If the next character would be a backslash (`\\`), would it
    /// be escaped or would it itself be viewed as escape character?
    /// (`\\\\`)
    pub fn backslash_escaped(&self) -> bool {
        self.double_quote || self.backslash
    }

    /// Calls `step` for every character in line and returns
    /// the resulting state
    pub fn process(line: &str) -> EscapingState {
        let mut state = EscapingState::new();
        for ch in line.chars() {
            state.step(ch);
        }
        state
    }
}

/// Splits a commandline into its components/arguments.
/// Works similar to `split_whitespace`.
///
/// The main difference to `split_whitespace` is:
/// It respects whitespace escaping (`"`, `'`, `\`) as well as escaping
/// of the escaping characters (`\"`, `'\'`, …).
/// Thus, strings (`"A B C"`) will show up as single arguments.
pub fn split(cmdline: &str) -> Vec<String> {
    let mut parts = vec![];
    let mut act = String::new();
    let mut state = EscapingState::new();
    for ch in cmdline.chars() {
        if ch.is_whitespace() && !state.whitespace_escaped() {
            if !act.is_empty() {
                parts.push(act);
                act = String::new();
            }
        } else {
            match ch {
                '"' => {
                    if state.doublequote_escaped() {
                        act.push(ch);
                    }
                }
                '\'' => {
                    if state.singlequote_escaped() {
                        act.push(ch);
                    }
                }
                '\\' => {
                    if state.backslash_escaped() {
                        act.push(ch);
                    }
                }
                ch => act.push(ch),
            }
        }
        state.step(ch);
    }
    if !act.is_empty() {
        parts.push(act);
    }
    parts
}

/// A completion callback must return a `Vec<String>` of
/// possibilities for the current argument.
/// 
/// For instance, if the possible flags for a subcommand are
/// `["--help, "--halt", "--destroy]` and the user typed in
/// `--h` before requesting completion, the completion function
/// has to return every word in the wordlist starting
/// with that prefix. (`["--halt", "--help"]`)
/// 
/// Imagine the following function.
/// It was simplified using the convenience function `prefix_completion`.
/// ```norun
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
/// ```
pub fn prefix_completion(word: &str, wordlist: &[&str]) -> Vec<String> {
    wordlist
        .iter()
        .filter(|&e| {
            (*e).starts_with(word)
            })
        .map(|s| s.to_string())
        .collect()
}
