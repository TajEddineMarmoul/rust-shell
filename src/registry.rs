use std::collections::HashMap;

use crate::libs::command::{Command, ResultCommand};

pub struct Registry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register_all(&mut self, commands: Vec<Box<dyn Command>>) {
        for command in commands {
            self.register(command);
        }
    }

    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name().to_string(), command);
    }

    pub fn execute(&self, commands: &[String]) -> (ResultCommand, String) {
        if commands.is_empty() {
            return (ResultCommand::Continue, "Empty".to_string());
        }

        let command = &commands[0];
        let args = &commands[1..];

        let Some(selected_command) = self.commands.get(command.as_str()) else {
            return (
                ResultCommand::Continue,
                format!("{}: command not found", command),
            );
        };

        selected_command.execute(args)
    }
}
