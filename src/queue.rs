use songbird::Songbird;
use std::sync::Arc;
use rand::Rng;
use serenity::model::{
  id::GuildId,
  channel::Message,
};
use serenity::client::Context;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct VoiceLineData {
  pub new_time: u64,
  pub prev_time: u64,
  pub time_spent: u64,
  pub manager: Arc<Songbird>,
  pub ctx: Context,
  pub msg: Message,
}


const VANAISA_ID: u64 = 857297760414728262;

pub async fn play_voiceline(data: VoiceLineData, guild_id: GuildId) {
  if let Some(handler_lock) = data.manager.get(guild_id) {
    let roll: i8 = rand::thread_rng().gen_range(1..10);

    let filename = format!(
      "mis{}.mp3", 
      if roll < 10 { 
        format!("0{}", roll)
      } else { 
        roll.to_string() 
      }
    );
    let path_str = format!("./audio/{}", filename);
    let path = std::path::Path::new(&path_str);
    let source = match songbird::ffmpeg(path).await {
      Ok(source) => source,
      Err(err) => {
        println!("Error: {:?}", err);
        if let Err(err) = data.msg.channel_id.say(&data.ctx.http, "ffmpeg error").await {
          println!("Error: {:?}", err);
        }
        return;
      },
    };

    let mut handler = handler_lock.lock().await;
    let _handle = handler.play_source(source);

    let mins = data.time_spent / 60;
    let secs = data.time_spent - mins * 60;

    let content = format!(
      "mis see on
      ||{}m {}s||", 
      mins, 
      secs 
    );

    if let Err(err) = data.msg.channel_id.say(&data.ctx.http, content).await {
      println!("Error: {:?}", err);
    }
  } else {
    return;
  }

  sleep(Duration::new(1, 500_000_000)).await;
  match data.ctx.cache.channel(VANAISA_ID).await {
    Some(channel) => {
      if let Err(err) = channel.id().say(&data.ctx.http, "(mis see on)").await {
        println!("Error posting in comms channel: {:?}", err);
      }
    },
    None => println!("Couldn't find comms channel"),
  }
}