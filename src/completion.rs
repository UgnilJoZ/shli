use crate::parse::split;

/// A (sub)command may have arbitrary arguments, which the `Prompt`
/// may describe to the user, when prompted for tab completion.
/// `name` and `description` are for informative purpose only.
pub struct ArbitraryArgument {
	pub name: String,
	pub description: String,
}

/// Concrete possible argument
/// 
/// A flag may have a number of mandatory arguments, which's
/// number is `num_arguments`. Only after `arguments.len()`
/// arguments after a flag, the next argument may be completed.
pub struct Flag {
	pub name: String,
	pub arguments: Vec<ArbitraryArgument>,
}

impl Flag {
	pub fn new(flag: &str) -> Flag {
		Flag {
			name: String::from(flag),
			arguments: vec!(),
		}
	}
}

/// A (sub)command may have arguments which we divide into
/// two categories:
/// * A fixed set of flags/arguments
/// * A free-text argument with a description
pub enum Argument {
	/// Concrete possible argument
	/// 
	/// A flag may have a number of mandatory arguments
	Flag (Flag),
	/// A (sub)command may have arbitrary arguments, which the `Prompt`
	/// may describe to the user, when prompted for tab completion.
	/// `name` and `description` are for informative purpose only.
	ArbitraryArgument (ArbitraryArgument),
}

impl From<&str> for Argument {
	fn from(flag: &str) -> Argument {
		Argument::Flag(Flag::new(flag))
	}
}

/// Possible (sub)command displayed in tab completion.
/// 
/// The arguments right from a (sub)command may be flags,
/// arbitrary arguments or even subcommands. They all can
/// be displayed or even tab-completed, when described in
/// this data structure's attributes.
pub struct Command {
	pub name: String,
	pub args: Vec<Argument>,
	pub subcommands: Vec<Command>,
}

impl Command {
	/// Create a new instance by specifying the command's name
	pub fn new(name: &str) -> Command {
		Command {
			name: String::from(name),
			args: vec!(),
			subcommands: vec!(),
		}
	}

	/// Add a subcommand to this command
	pub fn subcommand(mut self, cmd: Command) -> Command {
		self.subcommands.push(cmd);
		self
	}

	/// Add a concrete argument (e.g. a flag) to this command
	pub fn arg<T: Into<Argument>>(mut self, arg: T) -> Command {
		self.args.push(arg.into());
		self
	}
}

pub enum CompletionResult {
	None,
	Description(String),
	PossibilityList(Vec<String>),
}

fn command_names(commands: &[Command]) -> Vec<String> {
	let mut result = vec!();
	for cmd in commands {
		result.push(cmd.name.clone());
	}
	result
}

/// Researches where in the command tree we are at the end of `cmdline`.
fn active_command<'a>(cmdline: &Vec<String>, commands: &'a [Command]) -> Option<&'a Command> {
	let mut result = None;
	for component in cmdline {
		for command in commands {
			if *component == command.name {
				result = Some(command)
			}
		}
	}
	return result
}

/// Returns the possible arguments (flags, subvommands, …) of `cmd`as `CompletionResult`
fn get_possible_completions(cmd: &Command) -> CompletionResult {
	let mut list = vec!();
	for arg in &cmd.args {
		match arg {
			// If one argument is arbitrary, we can't return a fixed lists of arguments
			Argument::ArbitraryArgument(_) => return CompletionResult::Description(String::from("Various artists")),
			Argument::Flag(flag) => list.push(flag.name.clone()),
		}
	}
	for cmd in &cmd.subcommands {
		list.push(cmd.name.clone())
	}
	CompletionResult::PossibilityList(list)
}

pub fn complete(previous: &str, commands: &[Command]) -> CompletionResult {
	if previous.is_empty() {
		let possible_commands = command_names(commands);
		if possible_commands.is_empty() {
			return CompletionResult::None
		} else {
			return CompletionResult::PossibilityList(possible_commands)
		}
	} else {
		let mut components = split(previous);
		// If the last character is not whitespace, the user is still typing the last component (word).
		// Let's not take it into account when researching flags for the current command.
		// Instead, complete it.
		// Else, the last component is completely typed in.

		// Since previous is not empty, it has surely a last character.
		// That is why we can unwrap the Option here.
		let to_complete = if previous.chars().last().unwrap().is_whitespace() {
			// When the last character is whitespace, return a new component
			String::new()
		} else {
			// When the last char is not whitespace, the current (last) component has to be completed.

			// As `previous` is not empty and not ending with whitespace, one component has to exist.
			// Therefore we can unwrap the not-occuring error safely here.
			components.pop().unwrap()
		};
		
		let mut possibilities = if let Some(cmd) = active_command(&components, commands) {
			if let CompletionResult::PossibilityList(possibilities) = get_possible_completions(&cmd) {
				possibilities
			} else {
				return CompletionResult::Description(String::from("Various possible"))
			}
		} else if components.is_empty() {
			command_names(commands)
		} else {
			vec![]
		};

		possibilities.retain(|possibility| possibility.starts_with(&to_complete));
		return CompletionResult::PossibilityList(possibilities)
	}
}
