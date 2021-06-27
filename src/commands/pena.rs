use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};

use rand::Rng;
use regex::Regex;


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
    let rgx = Regex::new(r"[P|p][E|e][N|n][A|a]").unwrap();
    if rgx.is_match(&msg.content) {
      rand::thread_rng().gen_bool(0.03)
    } else {
      false
    }
  }

  async fn action(&self, ctx: Context, msg: Message) {
    for _ in 0..5 {
      let content = format!("<@{}> mis see on", msg.author.id);
      if let Err(err) = msg.channel_id.say(&ctx.http, content).await {
        error!("Error: {:?}", err);
      }
    }
    self.log(ctx, msg);
  }
}