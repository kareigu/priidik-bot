use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
  prelude::*,
};


pub struct PenaCommand {
  name: &'static str,
}

impl PenaCommand {
  pub fn new() -> Self {
    Self {
      name: "pena",
    }
  }
}

#[async_trait]
impl Command for PenaCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "pena"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    if let Err(err) = msg.channel_id.say(&ctx.http, "<@855177115104575518> mis see on").await {
      println!("Error: {:?}", err);
    }
    self.log(ctx, msg);
  }
}