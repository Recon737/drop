use std::str::FromStr;

use clap::Args;
use dialoguer::{Input, theme::ColorfulTheme};
use s3::{Bucket, Region, creds::Credentials};
use serde::{Deserialize, Serialize};

use crate::{config::configurable::Configurable, interactive_optional_variable, interactive_variable};


#[derive(Serialize, Deserialize, Args, Clone)]
pub struct S3ConfigCli {
    secret_key: Option<String>,
    key_id: Option<String>,
    region: Option<String>,
    bucket_name: Option<String>,
    endpoint: Option<String>,
}

impl From<S3ConfigCli> for S3Config {
    fn from(value: S3ConfigCli) -> Self {
        interactive_variable!(value, secret_key, "S3 Secret Key");
        interactive_variable!(value, key_id, "S3 Key ID");
        interactive_variable!(value, region, "S3 Region");
        interactive_variable!(value, bucket_name, "S3 Bucket Name");
        interactive_optional_variable!(value, endpoint, "S3 Endpoint (leave blank for none");
        Self {
            secret_key,
            key_id,
            region,
            bucket_name,
            endpoint,
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct S3Config {
    secret_key: String,
    key_id: String,
    region: String,
    bucket_name: String,
    endpoint: Option<String>,
}

impl S3Config {
    pub fn generate_bucket(&self) -> anyhow::Result<s3::Bucket> {
        let credentials =
            Credentials::new(Some(&self.key_id), Some(&self.secret_key), None, None, None)?;

        let region = if let Some(endpoint) = &self.endpoint {
            Region::Custom {
                region: self.region.clone(),
                endpoint: endpoint.clone(),
            }
        } else {
            Region::from_str(&self.region)?
        };

        let bucket = Bucket::new(&self.bucket_name, region, credentials)?;

        Ok(*bucket)
    }
}

impl Configurable for S3Config {
    fn configure(&self, config: &mut super::config::Config) {
        println!("Configuring S3Config with {:?}", self);
    }
}
