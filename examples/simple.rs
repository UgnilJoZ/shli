extern crate shli;
use shli::completion::Command;
use shli::{Prompt, Error};

fn main() {
    let mut p = Prompt::new(
        "> ".to_string(),
        vec![
            Command::new("print"),
            Command::new("echo"),
            Command::new("cat").arg("--help"),
            Command::new("exit"),
        ],
    );
    loop {
        // read_commandline does all the reading and tab completion
        match p.read_commandline() {
            Ok(line) => {
                println!("");
                match line.get(0).map(|s| s.as_str()) {
                    Some("exit") => break,
                    Some("print") | Some("echo") => {
                        if line.len() > 1 {
                            let output = line[1..]
                                .iter()
                                .map(|s| &**s)
                                .collect::<Vec<&str>>()
                                .join(" ");
                            println!("{}", output);
                        }
                    }
                    Some(cmd) => println!("Did not find '{}' command!", cmd),
                    None => {}
                }
            }
            Err(Error::CtrlD) => {
                    println!("exit");
                    break;
            }
            Err(Error::CtrlC) => println!("\nCtrl+C pressed."),
            Err(Error::IoError(e)) => println!("Reading error: {:?}", e),
        }
    }
}
