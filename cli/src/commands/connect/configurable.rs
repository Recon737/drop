use crate::commands::connect::config_option::ConfigOption;

pub trait Configure {
    async fn configure(self, name: &mut Option<String>) -> anyhow::Result<ConfigOption>;
}
