mod server;
mod models;

use server::lib::stopwatch::Stopwatch;
use server::rejections::NoUserAgent;
use server::routes;
use std::env;
use std::time::Duration;
use std::{convert::Infallible, error::Error, net::SocketAddr};
use tokio::time;
use tracing::{info, warn};
use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};

use crate::server::lib::postgres;
use crate::server::lib::images::CACHE;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let addr = format!("{}:{}", env::var("BIND_ADDRESS")?, env::var("BIND_PORT")?)
        .parse::<SocketAddr>()?;

    info!("Starting the API server...");

    postgres::pq_connect();
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

    let mut interval = time::interval(Duration::new(3600, 0));

    // Handler to renew the cache every hour
    tokio::spawn(async move {
        interval.tick().await;

        loop {
            interval.tick().await;

            tokio::spawn(rebuild_cache());
        }
    });

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
    server::lib::images::populate().await?;
    let cache_time = cache_timer.stop();
    info!("Image cache Built in {}ms", cache_time); 

    Ok(())
}

async fn rebuild_cache() {
    let cache_timer = Stopwatch::new();
    CACHE.write().ok().unwrap().clear();
    server::lib::images::populate().await.unwrap();
    let cache_time = cache_timer.stop();
    info!("Image cache Rebuilt in {}ms", cache_time);
}

async fn handle_rejection(rejection: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let message;
    let code: StatusCode;

    if rejection.is_not_found() {
        message = "NOT_FOUND";
        code = StatusCode::NOT_FOUND;
    } else if rejection.find::<NoUserAgent>().is_some() {
        message = "NO_HEADERS";
        code = StatusCode::BAD_REQUEST;
    } else {
        eprintln!("Unhandled rejection: {:?}", rejection);

        message = "INTERNAL_SERVER_ERROR";
        code = StatusCode::INTERNAL_SERVER_ERROR
    }

    Ok(Response::builder().status(code).body(message))
}
