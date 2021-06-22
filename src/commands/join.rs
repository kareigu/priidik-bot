use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};

pub struct JoinCommand {
  name: &'static str,
}

impl JoinCommand {
  pub fn new() -> Self {
    Self {
      name: "join",
    }
  }
}

#[async_trait]
impl Command for JoinCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "&join"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
      .voice_states.get(&msg.author.id)
      .and_then(|vs|vs.channel_id);

    let connect_to = match channel_id {
      Some(ch) => ch,
      None => {
        if let Err(err) = msg.reply(&ctx.http, "Mis see on").await {
          println!("Error: {:?}", err);
        }
        return;
      }
    };

    let manager = songbird::get(&ctx).await
      .expect("Songbird client error").clone();

    let _handler = manager.join(guild_id, connect_to).await;
    self.log(ctx, msg);
  }
}