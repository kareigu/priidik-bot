use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
  prelude::*,
};

pub struct VanaisaCommand {
  name: String,
}

impl VanaisaCommand {
  pub fn new() -> Self {
    Self {
      name: "vanaisa".to_string(),
    }
  }
}

#[async_trait]
impl Command for VanaisaCommand {
  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.author.id.to_string() == "855177115104575518"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
      println!("Error: {:?}", err);
    }
    println!("{}", self.name);
  }
}
