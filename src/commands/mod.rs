use std::collections::HashMap;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
  prelude::*,
};

pub mod vanaisa;
pub mod pena;

pub struct CommandList {
  pub list: Vec<Box<dyn Command + Send + Sync>>,
}

impl CommandList {
  pub fn new() -> Self {
    let pena_cmd = Box::new(pena::PenaCommand::new());
    let vanaisa_cmd = Box::new(vanaisa::VanaisaCommand::new());

    Self {
      list: vec![pena_cmd, vanaisa_cmd]
    }
  }
}

#[async_trait]
pub trait Command {
  fn requirement(&self, ctx: &Context, msg: &Message) -> bool;
  async fn action(&self, ctx: Context, msg: Message);
  fn name(&self) -> &'static str;
  fn log(&self, ctx: Context, msg: Message) {
    println!("{username}#{id} ran command {cmd_name}", 
      username = msg.author.name,
      id = msg.author.id,
      cmd_name = self.name(),
    )
  }
}

