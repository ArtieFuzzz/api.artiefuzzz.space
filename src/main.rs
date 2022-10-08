mod server;

use std::{convert::Infallible, error::Error, net::SocketAddr};
use server::routes;
use std::env;
use tracing::{info, warn};
use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};
use server::lib::stopwatch::Stopwatch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let addr = format!("{}:{}", env::var("BIND_ADDRESS")?, env::var("BIND_PORT")?)
        .parse::<SocketAddr>()?;

    let cache_timer = Stopwatch::new();
    // Build the Image cache
    server::lib::images::init().await?;
    let cache_time = cache_timer.stop();
    info!("Image cache Built in {}ms", cache_time);

    let index = warp::get().and_then(routes::index);
    let list_files = warp::path!("list_images")
        .and(warp::get())
        .and_then(routes::list_images);

    let routes = warp::any()
        .and(list_files.or(index))
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
