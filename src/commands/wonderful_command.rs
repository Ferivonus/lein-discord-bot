use serenity::builder::CreateCommand;

pub fn wonderful_register() -> CreateCommand {
    CreateCommand::new("wonderful_command").description("An amazing command")
}
