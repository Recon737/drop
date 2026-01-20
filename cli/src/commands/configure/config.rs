use crate::commands::configure::{config_option::ConfigOption, s3::S3Config};
use log::warn;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

const CONFIG_DIR: &str = "downpour/config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    items: HashMap<String, ConfigOption>,
    active_s3: Option<String>,
}
impl Config {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            active_s3: None,
        }
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let json = serde_json::to_string(self)?;
        let save_path = dirs::config_dir()
            .expect("Apparently your home directory doesn't exist") // Should probably formalise that error
            .join(CONFIG_DIR);
        fs::create_dir_all(save_path.parent().unwrap())?;
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
    pub fn add_item(&mut self, name: String, object: ConfigOption) {
        if matches!(object, ConfigOption::S3(..)) {
            self.active_s3 = Some(name.clone())
        }
        self.items.insert(name, object);
        self.save().expect("Failed to save config");
    }

    pub fn get_active_s3(&self) -> Option<S3Config> {
        if let Some(active_s3) = &self.active_s3 {
            self.items
                .iter()
                .filter_map(|(name, option)| {
                    if *name == *active_s3 {
                        match option {
                            ConfigOption::S3(s3_config) => Some(s3_config),
                            _ => {
                                warn!("Name {} is not of type 'S3'", name);
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
