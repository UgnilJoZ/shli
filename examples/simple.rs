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
        match p.read_commandline() {
            Ok(line) => {
                println!("");
                if ! line.is_empty() {
                    match line[0].as_str() {
                        "exit" => break,
                        "print" | "echo" => if line.len() > 1 {
                            let mut output = line[1].clone();
                            for w in &line[2..] {
                                output.push_str(&format!(" {}", w));
                            }
                            println!("{}", output);
                        }
                        cmd => println!("Did not find '{}' command!", cmd),
                    }
                }
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
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