use dotenv;
use std::env;
use tokio;

use serenity::{
  async_trait,
  client::bridge::gateway::GatewayIntents,
  model::{channel::Message, gateway::Ready},
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
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} running", ready.user.name);
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
    .await
    .expect("Error creating client");
  
  if let Err(err) = client.start().await {
    println!("Client error: {:?}", err);
  }
}
