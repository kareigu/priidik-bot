use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};

use rand::Rng;

pub struct VanaisaCommand {
  name: &'static str,
}

impl VanaisaCommand {
  pub fn new() -> Self {
    Self {
      name: "vanaisa",
    }
  }
}

#[async_trait]
impl Command for VanaisaCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.author.id.to_string() == "855177115104575518" && rand::thread_rng().gen_bool(0.2)
  }

  async fn action(&self, ctx: Context, msg: Message) {
    if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
      println!("Error: {:?}", err);
    }
    self.log(ctx, msg);
  }
}
