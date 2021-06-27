use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};


pub struct SecretCommand {
  name: &'static str,
}

impl SecretCommand {
  pub fn new() -> Self {
    Self {
      name: "secret",
    }
  }
}

#[async_trait]
impl Command for SecretCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "&secret" && msg.author.id == 128685552450011137
  }

  async fn action(&self, ctx: Context, msg: Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(&ctx).await
      .expect("Songbird client error").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
      let path = "./audio/Primogenitor_master-02.mp3";
      let source = match songbird::ffmpeg(path).await {
        Ok(source) => source,
        Err(err) => {
          error!("Error: {:?}", err);
          if let Err(err) = msg.channel_id.say(&ctx.http, "ffmpeg error").await {
            error!("Error: {:?}", err);
          }
          return;
        },
      };

      let mut handler = handler_lock.lock().await;
      let _handle = handler.play_source(source);
    } else {
      return;
    }

    self.log(ctx, msg);
  }
}