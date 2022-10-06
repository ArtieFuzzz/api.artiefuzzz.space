mod server;

use std::{convert::Infallible, error::Error, net::SocketAddr};

use server::routes;
use std::env;
use tracing::warn;
use warp::{
    self,
    http::{Response, StatusCode},
    Filter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let addr = format!(
        "{}:{}",
        env::var("BIND_ADDRESS")?,
        env::var("BIND_PORT")?
    )
    .parse::<SocketAddr>()?;

    let index = warp::get().and_then(routes::index);
    let routes = warp::any().and(index).recover(handle_rejection);

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
