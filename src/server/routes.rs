fn reply(status: u16, message: &str) -> impl warp::Reply {
  return warp::http::Response::builder()
    .status(status)
    .body(message.to_string())
}

pub async fn index() -> Result<impl warp::Reply, warp::Rejection> {
  Ok(reply(200, "OK"))
}
