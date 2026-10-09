use crate::shell_metadata::ShellState;

pub enum ResultCommand {
    Continue,
    Exit,
    Error,
}
pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String], shell: &mut ShellState) -> (ResultCommand, String);
}
