use crate::libs::standards::inject_module;
pub enum ResultCommand {
    Continue,
    Exit,
    Error,
}

pub type CommandFn = fn(&[String]) -> (ResultCommand, String);

pub fn execute_command(commands: Vec<String>) -> (ResultCommand, String) {
    if commands.is_empty() {
        return (ResultCommand::Continue, "Empty".to_string());
    }

    let standard_modules = inject_module();

    let command = &commands[0];
    let args = &commands[1..];

    let Some(selected_function) = standard_modules.get(&command.as_str()) else {
        return (ResultCommand::Error, "Something Bad Happened".to_string());
    };

    selected_function(args)
}
