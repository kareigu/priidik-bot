use serenity::prelude::RwLock;
use songbird::Songbird;
use std::sync::Arc;
use rand::Rng;
use serenity::model::{
  id::GuildId,
  channel::Message,
};
use serenity::client::Context;
use serenity::prelude::*;
use std::time::Duration;
use tokio::time::sleep;
use std::collections::HashMap;

pub struct Queue;

impl TypeMapKey for Queue {
  type Value = Arc<RwLock<HashMap<GuildId, VoiceLineData>>>;
}

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

pub async fn queue_loop(queue_data: Arc<RwLock<TypeMap>>) {
  let queue_lock = {
    let data = queue_data.read().await;
    data.get::<Queue>()
      .expect("Nothing in queue")
      .clone()
  };

  let mut queue = queue_lock.write().await;

  let current_time = crate::utils::get_current_time();

  for i in queue.clone() {
    let nt = i.1.new_time;
    if nt <= current_time {
      let secs_to_wait = rand::thread_rng().gen_range(3..1500);
      let data = update_times(
        i.1, 
        current_time, 
        secs_to_wait
      );
      
      queue.insert(i.0, data.clone());

      play_voiceline(data, i.0.into()).await;
    }
  }

  sleep(Duration::new(1, 0)).await;
}

fn update_times(
  mut data: VoiceLineData, 
  current_time: u64, 
  secs_to_wait: u64
) -> VoiceLineData {
  data.time_spent = current_time - data.prev_time;
  data.prev_time = data.new_time;
  data.new_time = current_time + secs_to_wait;
  data
}

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

    let content = format_message(data.time_spent);

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

fn format_message(time_spent: u64) -> String {
  let mins = time_spent / 60;
  let secs = time_spent - mins * 60;

  if secs == 0 {
    "mis see on".to_string()
  } else {
    format!(
      "mis see on
      ||{}m {}s||", 
      mins, 
      secs 
    )
  }
}