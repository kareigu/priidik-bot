use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
  prelude::*,
};


pub struct PenaCommand {
  name: String,
}

impl PenaCommand {
  pub fn new() -> Self {
    Self {
      name: "Pena".to_string(),
    }
  }
}

#[async_trait]
impl Command for PenaCommand {
  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "pena"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    if let Err(err) = msg.channel_id.say(&ctx.http, "<@855177115104575518> mis see on").await {
      println!("Error: {:?}", err);
    }
  }
}