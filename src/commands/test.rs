use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};
use songbird::Songbird;
use rand::Rng;
use tokio::time::sleep;
use std::sync::Arc;

pub struct TestCommand {
  name: &'static str,
}

impl TestCommand {
  pub fn new() -> Self {
    Self {
      name: "test",
    }
  }
}

#[async_trait]
impl Command for TestCommand {
  fn name(&self) -> &'static str {
    self.name
  }

  fn requirement(&self, _ctx: &Context, msg: &Message) -> bool {
    msg.content == "&test"
  }

  async fn action(&self, ctx: Context, msg: Message) {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(&ctx).await
      .expect("Songbird client error").clone();

    play_mis(ctx, manager, guild_id, msg).await;
  }
}

use std::future::Future;
use std::pin::Pin;
fn play_mis(
  ctx: Context, 
  manager: Arc<Songbird>, 
  guild_id: serenity::model::id::GuildId,
  msg: Message
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
  Box::pin(async move {
    if let Some(handler_lock) = manager.get(guild_id) {

      let roll: i8 = rand::thread_rng().gen_range(1..10);
  
      let filename = format!("mis{}.mp3", if roll < 10 { format!("0{}", roll)} else { roll.to_string() });
      let path_str = format!("./audio/{}", filename);
      let path = std::path::Path::new(&path_str);
      let source = match songbird::ffmpeg(path).await {
        Ok(source) => source,
        Err(err) => {
          println!("Error: {:?}", err);
          if let Err(err) = msg.channel_id.say(&ctx.http, "ffmpeg error").await {
            println!("Error: {:?}", err);
          }
          return;
        },
      };
  
      //println!("{:?}", source.seek_time(std::time::Duration::from_secs(0)));
      let mut handler = handler_lock.lock().await;
      let handle = handler.play_source(source);
      println!("{:?}", handle.metadata());
      if let Err(err) = msg.channel_id.say(&ctx.http, "mis see on").await {
        println!("Error: {:?}", err);
      }
    } else {
      return;
    }
  
    sleep(std::time::Duration::new(180, 0)).await;
    play_mis(ctx, manager, guild_id, msg).await;
  })
}