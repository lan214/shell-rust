#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let splits = line.trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let command = splits[0];

        match command {
            "exit" => break,
            "echo" => println!("{}", splits[1..].join(" ")),
            _ => println!("{command}: command not found"),
        }
    }
}
