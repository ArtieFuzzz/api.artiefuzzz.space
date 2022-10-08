use serde::{Serialize, Deserialize};
use warp::reply::{json};
use super::lib::images::CACHE;

#[derive(Serialize, Deserialize)]
struct ImageMessage {
  url: String
}

fn reply(status: u16, message: &str) -> impl warp::Reply {
  return warp::http::Response::builder()
    .status(status)
    .body(message.to_string())
}

fn reply_img(body: ImageMessage) -> impl warp::Reply {
  return json(&body)
}

pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
  Ok(reply(200, "OK"))
}

pub async fn random() -> Result<impl warp::Reply, warp::Rejection> {
  let rand = CACHE.read().ok().unwrap().random();

  Ok(reply_img(ImageMessage { url: rand }))
}

pub async fn random_meme() -> Result<impl warp::Reply, warp::Rejection> {
  let rand = CACHE.read().ok().unwrap().meme();

  Ok(reply_img(ImageMessage { url: rand }))
}
