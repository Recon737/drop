use crate::config::config::Config;

/// Trait which represents data which may be stored in `config_dir/downpour/config.json`
pub trait Configurable {
    fn configure(&self, config: &mut Config);
}

