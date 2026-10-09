pub enum ResultCommand {
    Continue,
    Exit,
    Error,
}
pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String]) -> (ResultCommand, String);
}
