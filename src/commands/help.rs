use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn help_run(_options: &[ResolvedOption]) -> String {
    "World of wires! with help!!".to_string()
}

pub fn help_register() -> CreateCommand {
    CreateCommand::new("help").description("A help comment for ferv bot")
}
