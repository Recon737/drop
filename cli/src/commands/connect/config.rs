use crate::{commands::{connect::{
    config_option::{ConfigOption, ConfigOptionCli},
    configurable::Configure,
    s3::S3Config,
}, upload::speedtest::Speedtest}, manifest::DepotManifest};
use dialoguer::{Confirm, theme::ColorfulTheme};
use futures::AsyncWriteExt;
use log::{debug, info, warn};
use opendal::Operator;
use serde::{Deserialize, Serialize};
use tokio_util::compat::FuturesAsyncWriteCompatExt;
use std::{collections::HashMap, fs, ops::Not};

const CONFIG_DIR: &str = "downpour/config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    configurations: HashMap<String, ConfigOption>,
    active_s3: Option<String>,
}
impl Config {
    pub fn new() -> Self {
        Self {
            configurations: HashMap::new(),
            active_s3: None,
        }
    }
    pub fn exists(&self, name: &String) -> bool {
        self.configurations.contains_key(name)
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
        self.configurations.insert(name, object);
        self.save().expect("Failed to save config");
    }

    pub fn get_active_s3(&self) -> Option<S3Config> {
        if let Some(active_s3) = &self.active_s3 {
            self.configurations
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
    pub fn get<T: AsRef<String>>(&self, name: T) -> Option<&ConfigOption> {
        self.configurations.get(name.as_ref())
    }
}

pub async fn manage_configuration(
    config: &mut Config,
    name: &String,
    option: &ConfigOptionCli,
) -> anyhow::Result<()> {
    if config.exists(&name) {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "An entry already exists with the name \"{}\". Would you like to overwrite it?",
                &name
            ))
            .interact()?;
        if !confirm {
            return Err(anyhow::anyhow!("User cancelled action"));
        }
    }
    let config_option = match option {
        ConfigOptionCli::S3(s3_config_cli) => s3_config_cli.clone().configure().await?,
    };
    config.add_item(name.clone(), config_option.clone());
    let operator = config_option.build()?;

    generate_speedtest(&operator).await?;
    generate_manifest(&operator).await?;

    Ok(())
}

async fn generate_speedtest(operator: &Operator) -> anyhow::Result<()> {
    if operator.exists("speedtest").await?.not() {
        info!("Speedtest already exists on Depot. Skipping speedtest upload...");
        return Ok(())
    }
    let mut writer = operator.writer("speedtest").await?.into_futures_async_write().compat_write();
    let mut reader = Speedtest::new();
    let written = tokio::io::copy(&mut reader, &mut writer).await?;
    debug!("Wrote {} bytes to {:?}", written, operator.info());
    writer.into_inner().close().await?;
    Ok(())
}
async fn generate_manifest(operator: &Operator) -> anyhow::Result<()> {
    info!("Manifest already exists on Depot. Skipping manifest upload...");
    if operator.exists("manifest.json").await?.not() {
        return Ok(())
    }
    let data = DepotManifest::new();
    operator.write("manifest.json", serde_json::to_string(&data)?).await?;

    Ok(())
}