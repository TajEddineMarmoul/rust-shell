use std::{path::PathBuf, vec};

use crate::{
    libs::command::{Command, ResultCommand},
    shell_metadata::ShellState,
};

pub fn inject_module() -> Vec<Box<dyn Command>> {
    let mut commands: Vec<Box<dyn Command>> = vec![];

    commands.push(Box::new(ExitCommand));
    commands.push(Box::new(EchoCommand));
    commands.push(Box::new(CdCommand));
    commands
}

pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &str {
        "exit"
    }

    fn execute(&self, _args: &[String], _shell: &mut ShellState) -> (ResultCommand, String) {
        (ResultCommand::Exit, "Exited Successfully".to_string())
    }
}

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &str {
        "echo"
    }

    fn execute(&self, args: &[String], _shell: &mut ShellState) -> (ResultCommand, String) {
        let joined = args.join(" ");

        (ResultCommand::Continue, joined)
    }
}

pub struct CdCommand;

impl Command for CdCommand {
    fn name(&self) -> &str {
        "cd"
    }

    fn execute(&self, args: &[String], shell: &mut ShellState) -> (ResultCommand, String) {
        let Some(selected_path_raw) = args.first() else {
            return (ResultCommand::Error, "Command Require a path".to_string());
        };

        let selected_path = PathBuf::from(selected_path_raw);

        let final_path = if selected_path.is_absolute() {
            selected_path
        } else {
            shell.cwd.join(selected_path)
        };

        match final_path.canonicalize() {
            Ok(resolved) => {
                shell.cwd = resolved;
                (ResultCommand::Continue, String::new())
            }
            Err(e) => (ResultCommand::Error, format!("cd failed : {e}")),
        }
    }
}
