use std::collections::HashMap;

use crate::commands::{CommandFn, ResultCommand};

pub fn inject_module() -> HashMap<&'static str, CommandFn> {
    let mut commands_package: HashMap<&'static str, CommandFn> = HashMap::new();
    commands_package.insert("exit", exit_shell);
    commands_package.insert("echo", echo);
    commands_package
}

fn exit_shell(_args: &[String]) -> (ResultCommand, String) {
    (ResultCommand::Exit, "Exited Successfully".to_string())
}

fn echo(args: &[String]) -> (ResultCommand, String) {
    let joined = args.join(" ");
    (ResultCommand::Continue, joined)
}
