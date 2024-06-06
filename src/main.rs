use anyhow::Context as _;
use rand::Rng;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        } else if msg.content.to_lowercase().contains("buffcorrell")
            || msg.content.to_lowercase().contains("buffy c")
        {
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
