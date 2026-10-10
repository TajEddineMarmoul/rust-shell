use std::{
    io::{self, Write},
    println,
};

pub mod libs;
mod registry;
pub mod shell_metadata;
use crate::{
    libs::{command::ResultCommand, standards::inject_module},
    shell_metadata::ShellState,
};
mod tokenizer;
fn main() {
    let mut loaded_registry = registry::Registry::new();

    loaded_registry.register_all(inject_module());

    loop {
        initiate_shell(loaded_registry.get_shell());

        let commands = get_commands();

        let (result_execution, result_comment) = loaded_registry.execute(&commands);
        match result_execution {
            ResultCommand::Exit => {
                println!("{result_comment}");
                break;
            }
            _ => {
                if !result_comment.is_empty() {
                    println!("{result_comment}");
                }
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

fn initiate_shell(shell: &ShellState) {
    let cwd = shell.cwd.display();
    print!(" shell: {cwd}");
    print!("> ");
    io::stdout().flush().unwrap();
}
