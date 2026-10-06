use std::collections::HashMap;

pub enum ResultCommand {
    Continue,
    Executed,
    Exit,
    Error,
}

pub fn execute_command(commands: Vec<String>) -> (ResultCommand, &'static str) {
    if commands.is_empty() {
        return (ResultCommand::Continue, "Empty");
    }

    let mut commands_package: HashMap<&str, fn(&[String]) -> (ResultCommand, &'static str)> =
        HashMap::new();
    commands_package.insert("exit", exit_shell);

    let command = &commands[0];
    let args = &commands[1..];

    let Some(selected_function) = commands_package.get(&command.as_str()) else {
        return (ResultCommand::Error, "Something Bad Happened");
    };

    selected_function(args)
}

fn exit_shell(_args: &[String]) -> (ResultCommand, &'static str) {
    (ResultCommand::Exit, "Exited Successfully")
}
