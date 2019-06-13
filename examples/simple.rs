use std::io::{stdin, stdout};
extern crate shli;
use shli::split;
use shli::read_commandline;

fn example_completion(line: &str) -> Vec<String> {
    let cmd = split(&line);
    if cmd.len() == 1 {
        ["print", "echo", "exit"]
        .iter()
        .filter(|&e| {
            (*e).starts_with(&cmd[0])
            })
        .map(|s| s.to_string())
        .collect()
    } else if cmd.len() == 0 {
        vec!("print".to_string(), "echo".to_string(), "exit".to_string())
    } else {
        vec!()
    }
}

fn main() {
    loop {
        let stdin = stdin();
        let line_result = read_commandline(stdin.lock(), &mut stdout(), example_completion);
        match line_result {
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
                        cmd => println!("{} hab ich jetzt nicht so gefunden!", cmd),
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