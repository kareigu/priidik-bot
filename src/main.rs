use dotenv;
use std::env;
use tokio::{self, time::sleep};

use songbird::{SerenityInit, Songbird};

use rand::{Rng};

use std::sync::Arc;

use serenity::{
  async_trait,
  client::Context,
  client::bridge::gateway::GatewayIntents,
  model::{channel::Message, gateway::Ready, gateway::Activity},
  prelude::*,
};

mod commands;
use commands::Command;
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



/*     else if msg.content == "pena" {
      if let Err(err) = msg.channel_id.say(&ctx.http, "<@855177115104575518> mis see on").await {
        println!("Error: {:?}", err);
      }
    } else if msg.content == "&join" {
      let guild = msg.guild(&ctx.cache).await.unwrap();
      let guild_id = guild.id;

      let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|vs|vs.channel_id);

      let connect_to = match channel_id {
        Some(ch) => ch,
        None => {
          if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
            println!("Error: {:?}", err);
          }
          return;
        }
      };

      let manager = songbird::get(&ctx).await
        .expect("Songbird client error").clone();

      let _handler = manager.join(guild_id, connect_to).await;
    } else if msg.content == "&leave" {
      let guild = msg.guild(&ctx.cache).await.unwrap();
      let guild_id = guild.id;

      let manager = songbird::get(&ctx).await
        .expect("Songbird client error").clone();

      if manager.get(guild_id).is_some() {
        if let Err(err) = manager.remove(guild_id).await {
          println!("Error {:?}", err);
        }
      } else {
        if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
          println!("Error: {:?}", err);
        }
      }
    } else if msg.content == "&test" {
      let guild = msg.guild(&ctx.cache).await.unwrap();
      let guild_id = guild.id;

      let manager = songbird::get(&ctx).await
        .expect("Songbird client error").clone();

      play_mis(ctx, manager, guild_id, msg).await;
    } */
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
