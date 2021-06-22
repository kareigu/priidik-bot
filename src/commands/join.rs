use std::{future::Future, pin::Pin};
use std::sync::Arc;
use crate::commands::Command;
use serenity::{
  async_trait,
  client::Context,
  model::channel::Message,
};
use songbird::Songbird;
use rand::Rng;
use tokio::time::sleep;

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
    play_voiceline(ctx.clone(), manager, guild_id, msg.clone()).await;
    self.log(ctx, msg);
  }
}


fn play_voiceline(
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
      let _handle = handler.play_source(source);
      if let Err(err) = msg.channel_id.say(&ctx.http, "mis see on").await {
        println!("Error: {:?}", err);
      }
    } else {
      return;
    }
    
    sleep(std::time::Duration::new(180, 0)).await;
    play_voiceline(ctx, manager, guild_id, msg).await;
  })
}