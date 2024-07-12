use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::Timestamp;
use serenity::prelude::*;

use sqlx::MySqlPool;
use std::env;
use std::error::Error;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                println!("Error sending message: {:?}", e);
            }
        }

        if msg.content == "!dm" {
            let footer = CreateEmbedFooter::new("Always connected in the Wired");
            let embed = CreateEmbed::new()
                .title("Greetings, I am Lain Iwakura")
                .description("Welcome to the Wired. Are you ready to explore the depths of the digital realm?")
                .image("attachment://Serial_Experiments_Lain_Lain.jpg")
                .fields(vec![
                    ("Reality vs. Wired", "Do you believe in the boundaries of reality?", true),
                    ("Connection", "We are all interconnected in the Wired...", true),
                    ("The Wired", "A place where thoughts become reality.", true),
                ])
                .field(
                    "Question for You",
                    "What secrets do you seek in the Wired?",
                    false,
                )
                .footer(footer)
                .timestamp(Timestamp::now());
            let builder = CreateMessage::new()
                .content("Hello, explorers of the Wired!")
                .embed(embed)
                .add_file(
                    CreateAttachment::path("./Serial_Experiments_Lain_Lain.jpg")
                        .await
                        .unwrap(),
                );
            let msg = msg.author.dm(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }

        if (msg.content == "!orkun") && (msg.author.id == 317767790029438988) {
            let builder = CreateMessage::new().content("iyi ki varsın abi seni çok seviom!");
            let dm = msg.author.dm(&ctx, builder).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {why:?}");
            }
        }

        if (msg.content == "!ender") && (msg.author.id == 305720245853880321) {
            // If the `utils`-feature is enabled, then model structs will have a lot of useful
            // methods implemented, to avoid using an often otherwise bulky Context, or even much
            // lower-level `rest` method.
            //
            // In this case, you can direct message a User directly by simply calling a method on
            // its instance, with the content of the message.
            let builder = CreateMessage::new().content("iyi ki varsın gardasım seni çok seviom!");
            let dm = msg.author.dm(&ctx, builder).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let token =
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment for discord token");

    // Get the database URL from the environment variables and create a MySQL pool
    let database_url = env::var("DATABASE_URL")
        .expect("Expected a token in the environment for mysql database url");
    let _pool = MySqlPool::connect(&database_url).await?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Error creating client");

    client.start().await?;

    Ok(())
}
