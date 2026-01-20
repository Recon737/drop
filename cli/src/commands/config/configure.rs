use crate::commands::config::config_option::ConfigOption;

pub trait Configurable {
    async fn configure(self) -> anyhow::Result<ConfigOption>;
}
