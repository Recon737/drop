use std::str::FromStr;

use clap::Args;
use s3::{Bucket, Region, creds::Credentials};
use serde::{Deserialize, Serialize};

use crate::{
    commands::config::{config_option::ConfigOption, configure::Configurable},
    interactive_optional_variable, interactive_variable,
};

#[derive(Args, Clone)]
pub struct S3ConfigCli {
    secret_key: Option<String>,
    key_id: Option<String>,
    region: Option<String>,
    bucket_name: Option<String>,
    endpoint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3Config {
    secret_key: String,
    key_id: String,
    region: String,
    bucket_name: String,
    endpoint: Option<String>,
}

impl Configurable for S3ConfigCli {
    async fn configure(self) -> anyhow::Result<ConfigOption> {
        interactive_variable!(self, secret_key, "S3 Secret Key");
        interactive_variable!(self, key_id, "S3 Key ID");
        interactive_variable!(self, region, "S3 Region");
        interactive_variable!(self, bucket_name, "S3 Bucket Name");
        interactive_optional_variable!(self, endpoint, "S3 Endpoint (leave blank for none");
        Ok(ConfigOption::S3(S3Config {
            secret_key,
            key_id,
            region,
            bucket_name,
            endpoint,
        }))
    }
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
