use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::prelude::*;

use serenity::builder::{CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::Timestamp;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "World of wires!").await?;

    Ok(())
}

#[command]
async fn dm(ctx: &Context, msg: &Message) -> CommandResult {
    let footer = CreateEmbedFooter::new("Always connected in the Wired");
    let embed = CreateEmbed::new()
        .title("Greetings, I am Lain Iwakura")
        .description(
            "Welcome to the Wired. Are you ready to explore the depths of the digital realm?",
        )
        .image("attachment://Serial_Experiments_Lain_Lain.jpg")
        .fields(vec![
            (
                "Reality vs. Wired",
                "Do you believe in the boundaries of reality?",
                true,
            ),
            (
                "Connection",
                "We are all interconnected in the Wired...",
                true,
            ),
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

    Ok(())
}

#[command]
async fn orkun(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id == 317767790029438988 {
        let builder = CreateMessage::new().content("iyi ki varsın abi seni çok seviom!");
        let dm = msg.author.dm(&ctx, builder).await;

        if let Err(why) = dm {
            println!("Error when direct messaging user: {why:?}");
        }
    } else {
        let builder = CreateMessage::new().content("Sen orkun değilsin.!");
        let dm = msg.author.dm(&ctx, builder).await;

        if let Err(why) = dm {
            println!("Error when direct messaging user: {why:?}");
        }
    }

    Ok(())
}

#[command]
async fn ender(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id == 305720245853880321 {
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
    } else {
        let builder = CreateMessage::new().content("Sen ender değilsin.!");
        let dm = msg.author.dm(&ctx, builder).await;

        if let Err(why) = dm {
            println!("Error when direct messaging user: {why:?}");
        }
    }

    Ok(())
}
