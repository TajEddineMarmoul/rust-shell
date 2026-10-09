use std::vec;

use crate::libs::command::{Command, ResultCommand};

pub fn inject_module() -> Vec<Box<dyn Command>> {
    let mut commands: Vec<Box<dyn Command>> = vec![];

    commands.push(Box::new(ExitCommand));
    commands.push(Box::new(EchoCommand));

    commands
}

pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &str {
        "exit"
    }

    fn execute(&self, _args: &[String]) -> (ResultCommand, String) {
        (ResultCommand::Exit, "Exited Successfully".to_string())
    }
}

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &str {
        "echo"
    }

    fn execute(&self, args: &[String]) -> (ResultCommand, String) {
        let joined = args.join(" ");
        (ResultCommand::Continue, joined)
    }
}
