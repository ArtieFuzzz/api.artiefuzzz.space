mod server;

use server::lib::stopwatch::Stopwatch;
use server::routes;
use std::env;
use std::{convert::Infallible, error::Error, net::SocketAddr};
use tracing::{info, warn};
use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let addr = format!("{}:{}", env::var("BIND_ADDRESS")?, env::var("BIND_PORT")?)
        .parse::<SocketAddr>()?;

    build_cache().await?;

    let index = warp::path::end().and(warp::get()).and_then(routes::index);
    let random_image = warp::path!("images" / "random")
        .and(warp::get())
        .and_then(routes::random);
    let meme_image = warp::path!("images" / "meme")
        .and(warp::get())
        .and_then(routes::random_meme);

    let routes = warp::any()
        .and(index.or(random_image).or(meme_image))
        .recover(handle_rejection);

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(routes).run(addr).await;

    Ok(())
}

async fn build_cache() -> Result<(), Box<dyn Error>> {
  let cache_timer = Stopwatch::new();
  server::lib::images::init().await?;
  let cache_time = cache_timer.stop();
  info!("Image cache Built in {}ms", cache_time);

  Ok(())
}

async fn handle_rejection(rejection: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let message;
    let code: StatusCode;

    if rejection.is_not_found() {
        message = "NOT_FOUND";
        code = StatusCode::NOT_FOUND;
    } else {
        eprintln!("Unhandled rejection: {:?}", rejection);

        message = "INTERNAL_SERVER_ERROR";
        code = StatusCode::INTERNAL_SERVER_ERROR
    }

    Ok(Response::builder().status(code).body(message))
}
