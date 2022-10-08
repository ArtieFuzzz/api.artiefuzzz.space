use s3::{creds::Credentials, Bucket, Region};
use std::{env::var, error::Error};

pub fn get_bucket() -> Result<Bucket, Box<dyn Error>> {
    let region = Region::Custom {
        region: "us-west".to_owned(),
        endpoint: "s3.us-west-000.backblazeb2.com".to_owned(),
    };

    let access_key = var("B2_KEY_ID")?;
    let key_id = var("B2_KEY")?;

    let credentials = Credentials::new(Some(&access_key), Some(&key_id), None, None, None)?;
    let bucket = Bucket::new(&var("B2_BUCKET_ID")?, region, credentials)?;

    Ok(bucket)
}
