use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
  prelude::*
};
use std::sync::Arc;

mod vanaisa;
mod pena;
mod join;
mod leave;
mod secret;

pub struct Commands;

impl TypeMapKey for Commands {
  type Value = Arc<CommandList>;
}

pub struct CommandList {
  pub list: Vec<Box<dyn Command + Send + Sync>>,
}

impl CommandList {
  pub fn new() -> Self {
    Self {
      list: vec![
        Box::new(pena::PenaCommand::new()), 
        Box::new(vanaisa::VanaisaCommand::new()),
        Box::new(join::JoinCommand::new()),
        Box::new(leave::LeaveCommand::new()),
        Box::new(secret::SecretCommand::new()),
      ]
    }
  }
}


#[async_trait]
pub trait Command {
  fn requirement(&self, ctx: &Context, msg: &Message) -> bool;
  async fn action(&self, ctx: Context, msg: Message);
  fn name(&self) -> &'static str;
  fn log(&self, _ctx: Context, msg: Message) {
    info!("{username}#{id} ran command {cmd_name}", 
      username = msg.author.name,
      id = msg.author.id,
      cmd_name = self.name(),
    )
  }
}

