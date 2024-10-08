use anyhow::Context as _;
use rand::Rng;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{
    client::{Client, EventHandler},
    prelude::GatewayIntents,
};
use shuttle_runtime::SecretStore;
use tracing::{error, info};

// Event related imports to detect track creation failures.

// To turn user URLs into playable audio, we'll use yt-dlp.

// Import the `Context` to handle commands.
use serenity::client::Context;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if is_a_gif_response(&msg.content) && !msg.author.bot {
            send_random_gif(ctx, msg).await;
        } else if msg.content.as_str() == "big b jam" && !msg.author.bot {
            let msg = msg.channel_id.say(&ctx.http, "Ha no").await;
            if let Err(e) = msg {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}

async fn send_random_gif(ctx: Context, msg: Message) {
    let random_number: i32 = rand::thread_rng().gen_range(0..12);
    let gif_name = format!("./media/buffy-dance{}.gif", random_number);
    let builder = CreateMessage::new()
        .content("LETS GET IT")
        .add_file(CreateAttachment::path(gif_name).await.unwrap());
    let msg = msg.channel_id.send_message(&ctx.http, builder).await;
    if let Err(e) = msg {
        error!("Error sending message: {:?}", e);
    }
}

fn is_a_gif_response(string: &str) -> bool {
    string.to_lowercase().contains("buffcorrell")
        || string.to_lowercase().contains("buffy c")
        || string.contains("LETS GET IT")
}
