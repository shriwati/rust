#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;


fn split_command(input: &str) -> (&str,&str) {
    let commands: Vec<&str> = input.split_whitespace().collect();
    let c=commands[0];
    let t = if commands.len() > 1 {
        commands[1]
    }else { commands[0]};
    (c,t)
}
fn main() {
    loop {

        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let cmd=split_command(input.trim());
        match cmd {
            ("exit",_) => std::process::exit(0),
            ("type",t) => {
                match t {
                    "echo" | "type" | "exit" => {
                        let is_command = Command::new("type").arg(t).output().map(|o| o.status.success()).unwrap_or(false);
                        if is_command {
                            println!("{} is a shell builtin", t);
                        } else {
                            println!("{} not found", t)
                        }
                    }
                    _ => println!("{} not found", t)
                }
            }
            _ => println!("{}: command not found", input.trim()),
        };
        input.clear();
    }
}