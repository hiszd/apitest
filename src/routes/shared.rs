use rocket::futures::StreamExt;

use crate::rocket::futures::SinkExt;
use crate::{Topic, STATE};

#[get("/sock")]
pub fn sock<'r>(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'r> {
  ws.channel(move |mut stream| {
    Box::pin(async move {
      let mut id: String = String::new();
      let mut v: Vec<Topic> = Vec::new();
      println!("Starting stream");
      while let Some(message) = stream.next().await {
        if let Ok(msg) = message {
          let mut dn: bool = false;
          let ms = msg.to_string();
          let m = ms.split(",").collect::<Vec<&str>>();
          m.iter().for_each(|m| {
            if let Ok(t) = Topic::try_from(m.to_string()) {
              v.push(t);
              dn = true;
            }
          });
          if !v.is_empty() {
            id = STATE.lock().await.subscribe("stream", v.clone());
          }
          println!(
            "Subscribed to: {}",
            v.iter().fold(String::new(), |acc, tpc| {
              let t = tpc.clone();
              if acc.is_empty() {
                String::from(t)
              } else {
                format!("{}, {}", acc, String::from(t))
              }
            })
          );
          if dn {
            break;
          }
        }
      }
      loop {
        for tpc in v.clone() {
          let update_topics = STATE.lock().await.check_subscriber(&id, tpc);
          if !update_topics.is_empty() {
            for t in update_topics {
              stream
                .send(rocket_ws::Message::text(String::from(t)))
                .await
                .unwrap();
            }
          }
        }
      }
    })
  })
}
