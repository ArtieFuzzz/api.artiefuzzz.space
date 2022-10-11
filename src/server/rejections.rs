use warp::reject::Reject;

// TODO
#[derive(Debug)]
pub struct NoUserAgent;

impl Reject for NoUserAgent {}