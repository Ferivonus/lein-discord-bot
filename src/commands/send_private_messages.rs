use serenity::all::Timestamp;
use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateMessage,
};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::model::colour::Colour;
use serenity::model::id::ChannelId;
use serenity::model::id::UserId;
use serenity::prelude::Context;

pub async fn send_to_channel_run(options: &[ResolvedOption<'_>], ctx: &Context) -> String {
    let mut channel_id: Option<ChannelId> = None;
    let mut message_content: Option<String> = None;

    // Kullanıcının sağladığı mesajı ve kanal kimliğini al
    for option in options {
        match option.name {
            "channel_id" => {
                if let ResolvedValue::Integer(id) = option.value {
                    channel_id = Some(ChannelId::new(id as u64));
                }
            }
            "message" => {
                if let ResolvedValue::String(ref content) = option.value {
                    message_content = Some(content.to_string());
                }
            }
            _ => {}
        }
    }

    if let (Some(channel_id), Some(content)) = (channel_id, message_content) {
        let builder = CreateMessage::new().content(content.clone());

        if let Err(why) = channel_id.send_message(&ctx.http, builder).await {
            println!("Error sending message: {:?}", why);
            return format!(
                "Failed to send the message to channel with ID {}.",
                channel_id
            );
        }
        format!(
            "Message sent successfully to channel with ID {}.",
            channel_id
        )
    } else {
        "Please provide a valid channel ID and message.".to_string()
    }
}

pub fn send_to_channel_register() -> CreateCommand {
    CreateCommand::new("send_message")
        .description("Send a message to a specific channel")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "channel_id",
                "The channel ID to send the message to",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "The message to send")
                .required(true),
        )
}

pub async fn send_to_user_by_user_id_run(options: &[ResolvedOption<'_>], ctx: &Context) -> String {
    let mut user_id: Option<UserId> = None;
    let mut message_content: Option<String> = None;

    // Kullanıcının sağladığı mesaj ve kullanıcı kimliğini al
    for option in options {
        match option.name {
            "id" => {
                if let ResolvedValue::Integer(id) = option.value {
                    user_id = Some(UserId::new(id as u64));
                }
            }
            "message" => {
                if let ResolvedValue::String(ref content) = option.value {
                    message_content = Some(content.to_string());
                }
            }
            _ => {}
        }
    }

    if let (Some(id), Some(content)) = (user_id, message_content) {
        let footer = CreateEmbedFooter::new("This is a footer");
        let embed = CreateEmbed::new()
            .title("DM Message")
            .description("This is an embedded message")
            .field("Message", content.clone(), false)
            .footer(footer)
            .colour(Colour::BLUE)
            .timestamp(Timestamp::now());

        let builder = CreateMessage::new().embed(embed).content(content.clone());

        if let Err(why) = id
            .to_user(&ctx.http)
            .await
            .unwrap()
            .dm(&ctx.http, builder)
            .await
        {
            println!("Error sending message: {:?}", why);
            return format!("Failed to send the message to user with ID {}.", id);
        }
        format!("Message sent successfully to user with ID {}.", id)
    } else {
        "Please provide a valid user ID and message.".to_string()
    }
}

pub fn send_to_user_by_user_id_register() -> CreateCommand {
    CreateCommand::new("send_dm_message") // Komut adı "_" ile ayrılmış şekilde düzeltildi
        .description("Send a DM message to a user by ID")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "The message to send")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "id",
                "The user ID to send the message to",
            )
            .required(true),
        )
}
