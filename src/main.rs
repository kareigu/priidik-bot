use dotenv;
use std::env;
use tokio::{self};

use songbird::SerenityInit;
use std::sync::Arc;

use serenity::{
  async_trait,
  client::Context,
  client::bridge::gateway::GatewayIntents,
  model::{channel::Message, gateway::Ready, gateway::Activity},
  prelude::*,
};

mod commands;
use commands::CommandList;

struct Handler;

struct Commands;

impl TypeMapKey for Commands {
  type Value = Arc<CommandList>;
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    let commands = {
      let data_read = ctx.data.read().await;
      data_read.get::<Commands>().expect("No Commands in TypeMap storage").clone()
    };

    for cmd in &commands.as_ref().list {
      if cmd.requirement(&ctx, &msg)  {
        cmd.action(ctx, msg).await;
        break;
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    let activity = Activity::watching("vanaisa");
    ctx.set_activity(activity).await;
    println!("{}#{} running", ready.user.name, ready.user.discriminator);
  }
}


#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let token = env::var("TOKEN")
    .expect("Missing bot token in environment");

  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .intents(
      GatewayIntents::GUILDS | 
      GatewayIntents::GUILD_MESSAGES | 
      GatewayIntents::GUILD_VOICE_STATES
    )
    .register_songbird()
    .await
    .expect("Error creating client");

  {
    let mut data = client.data.write().await;

    data.insert::<Commands>(Arc::new(CommandList::new()));
  }
  
  if let Err(err) = client.start().await {
    println!("Client error: {:?}", err);
  }
}
