extern crate shli;
use shli::completion::Command;
use shli::Prompt;

fn main() {
    let mut p = Prompt::new("> ".to_string(), vec!(
        Command::new("print"),
        Command::new("echo"),
        Command::new("cat").arg("--help"),
        Command::new("exit")
    ));
    loop {
        // read_commandline does all the reading and tab completion
        match p.read_commandline() {
            Ok(line) => {
                println!("");
                match line.get(0).map(|s| s.as_str()) {
                    Some("exit") => break,
                    Some("print") | Some("echo") => if line.len() > 1 {
                        let mut output = line[1].clone();
                        for w in &line[2..] {
                            output.push_str(&format!(" {}", w));
                        }
                        println!("{}", output);
                    }
                    Some(cmd) => println!("Did not find '{}' command!", cmd),
                    None => {}
                }
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
                        // EOF is not really unexpected here.
                        println!("exit");
                        break;
                    }
                    std::io::ErrorKind::Other => {
                        println!("\nCtrl+C pressed.");
                    }
                    _ => {
                        println!("Reading error: {:?}", e);
                    }
                };
            }
        }
    }
}