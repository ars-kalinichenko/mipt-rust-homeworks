use super::command::Command;

#[derive(Debug)]
pub struct Step {
    #[allow(unused)]
    name: String,
    commands: Vec<Command>,
}

impl Step {
    pub fn new(name: String, commands: Vec<Command>) -> Self {
        Self { name, commands }
    }

    #[allow(unused)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn commands(&self) -> &[Command] {
        self.commands.as_slice()
    }
}
