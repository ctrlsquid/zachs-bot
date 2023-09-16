mod slash_commands;

use poise::serenity_prelude as serenity;
use rand::{Rng, thread_rng};
use zachsbot::{debug, info};
use crate::slash_commands::dice_roll::roll;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Initialize tracing
    zachsbot::init_tracing();

    debug!("Initializing framework");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roll()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    info!("Starting bot connection to Discord");
    framework.run().await.unwrap();
}