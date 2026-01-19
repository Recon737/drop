use std::str::FromStr;

use s3::{Bucket, Region, creds::Credentials};

pub struct Config {
    items: Vec<ConfigItem>,
    active_s3: Option<String>,
}
pub struct ConfigItem {
    name: String,
    config_option: ConfigOption,
}
enum ConfigOption {
    S3(S3Config),
}
pub struct S3Config {
    secret_key: String,
    key_id: String,
    region: String,
    bucket_name: String,
    endpoint: Option<String>,
}
impl Config {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            active_s3: None,
        }
    }
    pub fn get_active_s3(&self) -> Option<&S3Config> {
        if let Some(active_s3) = &self.active_s3 {
            self.items
                .iter()
                .filter_map(|item| {
                    if item.name == *active_s3 {
                        match &item.config_option {
                            ConfigOption::S3(s3_config) => Some(s3_config),
                        }
                    } else {
                        None
                    }
                })
                .next()
        } else {
            None
        }
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
