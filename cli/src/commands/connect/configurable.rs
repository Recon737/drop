use crate::commands::connect::config_option::ConfigOption;

pub trait Configure {
    async fn configure(self) -> anyhow::Result<ConfigOption>;
}
