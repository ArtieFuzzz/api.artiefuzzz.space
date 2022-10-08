use super::lib::images::CACHE;


fn reply(status: u16, message: &str) -> impl warp::Reply {
  return warp::http::Response::builder()
    .status(status)
    .body(message.to_string())
}

pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
  Ok(reply(200, "OK"))
}

pub async fn list_images() -> Result<impl warp::Reply, warp::Rejection> {
  // Return JSON instead of Text (After all this is a test)
  let rand = CACHE.read().ok().unwrap().random_random();

  Ok(reply(200, &rand))
}
