use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::Region;
use std::env::var;
use std::error::Error;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct ImageCache {
    random: Vec<String>,
}

impl ImageCache {
    fn new() -> Self {
        return ImageCache { random: Vec::new() };
    }

    pub fn random_random(&self) -> String {
        return self
            .random
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_owned();
    }
}

lazy_static! {
    pub static ref CACHE: Arc<RwLock<ImageCache>> = Arc::new(RwLock::new(ImageCache::new()));
}

pub async fn init() -> Result<(), Box<dyn Error>> {
    let region = Region::Custom {
        region: "us-west".to_owned(),
        endpoint: "s3.us-west-000.backblazeb2.com".to_owned(),
    };

    let access_key = var("B2_KEY_ID")?;
    let key_id = var("B2_KEY")?;

    let credentials = Credentials::new(Some(&access_key), Some(&key_id), None, None, None)?;
    let bucket = Bucket::new(&var("B2_BUCKET_ID")?, region, credentials)?;

    let results = bucket
        .list_page("".to_string(), None, None, None, None)
        .await?;

    // TODO: Filter each result by their category (i.e random/ = random)
    for result in results.0.contents {
        let object = result.key.split("/").collect::<Vec<_>>();

        match object[0] {
            "random" => CACHE.write().ok().unwrap().random.push(format!(
                "https://img.artiefuzzz.space/random/{}",
                object[1].to_string()
            )),
            _ => (),
        }
    }

    Ok(())
}
