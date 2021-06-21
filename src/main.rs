use dotenv;
use std::env;
use tokio;

use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "vanamehe" {
      if let Err(err) = msg.channel_id.say(&ctx.http, "Vanaisa vanasia mis se on").await {
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
    .await
    .expect("Error creating client");
  
  if let Err(err) = client.start().await {
    println!("Client error: {:?}", err);
  }
}
