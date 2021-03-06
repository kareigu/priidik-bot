use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};

pub struct LeaveCommand {
  name: &'static str,
}

impl LeaveCommand {
  pub fn new() -> Self {
    Self {
      name: "leave",
    }
  }
}

#[async_trait]
impl Command for LeaveCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "&leave"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(&ctx).await
      .expect("Songbird client error").clone();

    if manager.get(guild_id).is_some() {
      if let Err(err) = manager.remove(guild_id).await {
        error!("Error {:?}", err);
      }
    } else {
      if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
        error!("Error: {:?}", err);
      }
    }
    {
      let queue_lock = {
        let data = ctx.data.read().await;
        data.get::<crate::Queue>()
          .expect("No queue")
          .clone()
      };
    
    
      let mut queue = queue_lock.write().await;
      queue.remove(&guild_id.into());
    }
    self.log(ctx, msg);
  }
}