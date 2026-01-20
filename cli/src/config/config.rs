use std::{fs, str::FromStr};

use clap::Subcommand;
use dialoguer::{Input, theme::ColorfulTheme};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::config::{
    s3::{S3Config, S3ConfigCli},
    server::ServerConfig,
};

const CONFIG_DIR: &str = "downpour/config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    items: Vec<ConfigItem>,
    active_s3: Option<String>,
}
impl Config {
    pub fn save(&self) -> anyhow::Result<()> {
        let json = serde_json::to_string(self)?;
        let save_path = dirs::config_dir()
            .expect("Apparently your home directory doesn't exist") // Should probably formalise that error
            .join(CONFIG_DIR);
        fs::write(save_path, json)?;
        Ok(())
    }
    pub fn read() -> Self {
        let save_path = dirs::config_dir()
            .expect("Apparently your home directory doesn't exist") // Should probably formalise that error
            .join(CONFIG_DIR);
        if fs::exists(&save_path).expect(&format!("Could not read save path {:#?}", &save_path)) {
            serde_json::from_str(&fs::read_to_string(save_path).unwrap()).unwrap()
        } else {
            Config::new()
        }
    }
    pub fn add_item(&mut self, item: ConfigItem) {
        if matches!(item.config_option, ConfigOption::S3(..)) {
            self.active_s3 = Some(item.name.clone())
        }
        self.items.push(item);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigItem {
    name: String,
    config_option: ConfigOption,
}
#[derive(Subcommand, Serialize, Deserialize)]
pub enum ConfigOption {
    Server(ServerConfig),
    S3(S3ConfigCli),
}

impl Config {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            active_s3: None,
        }
    }
    pub fn get_active_s3(&self) -> Option<S3Config> {
        if let Some(active_s3) = &self.active_s3 {
            self.items
                .iter()
                .filter_map(|item| {
                    if item.name == *active_s3 {
                        match &item.config_option {
                            ConfigOption::S3(s3_config) => Some(s3_config),
                            _ => {
                                warn!("Name {} is not of type 'S3'", item.name);
                                None
                            }
                        }
                    } else {
                        None
                    }
                })
                .next()
                .cloned()
                .map(|c| c.into())
        } else {
            None
        }
    }
}

