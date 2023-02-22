pub struct Command {
    command: String,
    arguments: Vec<String>,
}
impl ToString for Command {
    fn to_string(&self) -> String {
        let mut command = self.command.clone();
        for argument in &self.arguments {
            command.push_str(&format!(" {}", argument));
        }
        command
    }
}

