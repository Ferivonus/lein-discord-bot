#![allow(deprecated)] // We recommend migrating to poise, instead of using the standard command framework.

use dotenv::dotenv;
use serenity::all::standard::Configuration;
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::Timestamp;
use serenity::prelude::EventHandler;
use serenity::prelude::*;

use sqlx::MySqlPool;
use std::env;
use std::error::Error;

mod commands;

use std::collections::HashSet;
use std::sync::Arc;
use tracing_subscriber;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::gateway::ShardManager;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;

use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::owner::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct Bot;

#[group]
#[commands(ping, hello, dm, orkun, ender, multiply, quit)]
struct General;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    dotenv().ok();
    let token =
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment for discord token");

    // Get the database URL from the environment variables and create a MySQL pool
    let database_url = env::var("DATABASE_URL")
        .expect("Expected a token in the environment for mysql database url");
    let _pool = MySqlPool::connect(&database_url).await?;

    let http = Http::new(&token);

    // Fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().owners(owners).prefix("--"));

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Bot)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
