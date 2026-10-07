use std::{
    io::{self, Write},
    println,
};

mod commands;
pub mod libs;

use crate::commands::{ResultCommand, execute_command};
mod tokenizer;
fn main() {
    loop {
        initiate_shell();

        let commands = get_commands();

        let (result_execution, result_comment) = execute_command(&commands);
        match result_execution {
            ResultCommand::Exit => {
                println!("{result_comment}");
                break;
            }
            _ => {
                println!("{result_comment}");
            }
        }
    }
}

fn get_commands() -> Vec<String> {
    let input = read_input();

    let commands: Vec<String> = tokenizer::tokenize(&input);
    commands
}

fn read_input() -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn initiate_shell() {
    print!("> ");
    io::stdout().flush().unwrap();
}
