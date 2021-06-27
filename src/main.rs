use dotenv;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::{self, sync::RwLock};

use songbird::SerenityInit;
use std::sync::Arc;
use std::collections::HashMap;

use serenity::{
  async_trait,
  client::Context,
  client::bridge::gateway::GatewayIntents,
  model::{channel::Message, gateway::Ready, gateway::Activity, id::GuildId},
  prelude::*,
};

mod commands;
use commands::CommandList;

mod queue;
mod utils;

struct Handler {
  loop_running: AtomicBool,
}

struct Commands;

impl TypeMapKey for Commands {
  type Value = Arc<CommandList>;
}

struct Queue;


impl TypeMapKey for Queue {
  type Value = Arc<RwLock<HashMap<GuildId, queue::VoiceLineData>>>;
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    let commands = {
      let data_read = ctx.data.read().await;
      data_read.get::<Commands>()
        .expect("No Commands in TypeMap storage")
        .clone()
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

  async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
    if !self.loop_running.load(Ordering::Relaxed) {
      let data = ctx.data.clone();

      let queue_data = Arc::clone(&data);
      tokio::spawn(async move {
        loop {
          queue::queue_loop(Arc::clone(&queue_data)).await;
        }
      });

      self.loop_running.swap(true, Ordering::Relaxed);
    }
  }
}


#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let token = env::var("TOKEN")
    .expect("Missing bot token in environment");

  let mut client = Client::builder(&token)
    .event_handler(Handler {
      loop_running: AtomicBool::new(false),
    })
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
    data.insert::<Queue>(Arc::new(RwLock::new(HashMap::new())));

  }
  
  if let Err(err) = client.start().await {
    println!("Client error: {:?}", err);
  }
}