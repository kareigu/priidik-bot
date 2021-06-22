use dotenv;
use std::env;
use tokio;

use songbird::SerenityInit;

use serenity::{
  async_trait,
  client::Context,
  client::bridge::gateway::GatewayIntents,
  model::{channel::Message, gateway::Ready, gateway::Activity},
  prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.author.id.to_string() == "855177115104575518" {
      if let Err(err) = msg.reply(&ctx.http, "Vanaisa vanaisa mis see on").await {
        println!("Error: {:?}", err);
      }
    } else if msg.content == "pena" {
      if let Err(err) = msg.channel_id.say(&ctx.http, "<@855177115104575518> mis see on").await {
        println!("Error: {:?}", err);
      }
    } else if msg.content == "pena100" {
      for _ in 0..100 {
        if let Err(err) = msg.channel_id.say(&ctx.http, "<@855177115104575518> mis see on").await {
          println!("Error: {:?}", err);
        }
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

      if let Some(handler_lock) = manager.get(guild_id) {

        let path = std::path::Path::new("./audio/Primogenitor_master-02.mp3");
        let source = match songbird::ytdl("https://www.youtube.com/watch?v=FM_-DjMlnzE").await {
          Ok(source) => source,
          Err(err) => {
            println!("Error: {:?}", err);
            if let Err(err) = msg.channel_id.say(&ctx.http, "ffmpeg error").await {
              println!("Error: {:?}", err);
            }
            return;
          },
        };

        //println!("{:?}", source.seek_time(std::time::Duration::from_secs(0)));
        let mut handler = handler_lock.lock().await;
        let handle = handler.play_source(source);
        println!("{:?}", handle.metadata());
        if let Err(err) = msg.channel_id.say(&ctx.http, "mis see on").await {
          println!("Error: {:?}", err);
        }
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
    .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
    .register_songbird()
    .await
    .expect("Error creating client");
  
  if let Err(err) = client.start().await {
    println!("Client error: {:?}", err);
  }
}
