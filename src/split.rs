/// Simple state machine for processing escaping within command line strings.
///
/// An example usage:
/// ```
/// use shli::split::EscapingState;
///
/// // This will build the string `some_command "some string\"" "some other string`
/// let cmdline = "some_command \"some string\\\"\" \"some other string";
/// let mut state = EscapingState::process(cmdline);
///
/// // This assertion means that a following whitespace would be escaped.
/// // A finished command line string should not pass this assertion.
/// // This will succeed, because we made an error at `"some other string`.
/// assert!(state.whitespace_escaped())
/// ```
#[derive(Debug, Default)]
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

    /// If the next character would be a backslash (`\`), would it
    /// be escaped or would it itself be viewed as escape character?
    /// (`\\`)
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
        if !state.whitespace_escaped() && ch.is_whitespace() {
            if !act.is_empty() {
                parts.push(act);
                act = String::new();
            }
            continue;
        }

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
        state.step(ch);
    }

    if !act.is_empty() {
        parts.push(act);
    }
    parts
}

pub fn ends_with_whitespace(text: &str) -> bool {
    if let Some(ch) = text.chars().last() {
        ch.is_whitespace()
    } else {
        false
    }
}
