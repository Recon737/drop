use clap::Args;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use anyhow::{Result, anyhow};
use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use reqwest::Client;
use url::Url;

use crate::commands::config::{config_option::ConfigOption, configure::Configurable};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    url: String,
    token: String,
}
#[derive(Args, Clone)]
pub struct ServerConfigCli {
    /// Endpoint of the Drop server
    url: String,
    #[arg(short, long)]
    token: Option<String>,
}

const TOKEN_CREATE_PAYLOAD: &str =
    "eyJuYW1lIjoiZG93bnBvdXIgKGNsaSkiLCJhY2xzIjpbImRlcG90Om5ldyJdfQ==";

impl Configurable for ServerConfigCli {
    async fn configure(self) -> anyhow::Result<ConfigOption> {
        let base_url = Url::parse(&self.url)?;
        let mut token_create_url = base_url.join("/admin/settings/tokens")?;
        {
            let mut query = token_create_url.query_pairs_mut();
            query.append_pair("payload", TOKEN_CREATE_PAYLOAD);
        };

        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Open \"{}\" in your default browser?",
                token_create_url.as_str()
            ))
            .interact()?;

        if !confirm {
            return Err(anyhow!("User cancelled action"));
        }

        webbrowser::open(token_create_url.as_str())?;

        let token: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("API token")
            .interact_text()?;

        validate_configuration(&self.url, &token).await?;

        Ok(ConfigOption::Server(ServerConfig {
            url: self.url,
            token,
        }))
    }
}

static CLIENT: LazyLock<Client> = LazyLock::new(|| reqwest::Client::new());
const REQUIRED_ACLS: [&str; 1] = ["depot:new"];

pub async fn validate_configuration(url: &str, token: &str) -> Result<()> {
    let base_url = Url::parse(&url)?;
    let token_check_url = base_url.join("/api/v1/token")?;

    let acl_check = CLIENT
        .get(token_check_url)
        .bearer_auth(token)
        .send()
        .await?;

    if !acl_check.status().is_success() {
        return Err(anyhow!(
            "ACL check failed with response code: {}",
            acl_check.status()
        ));
    }

    let acls: Vec<String> = acl_check.json().await?;

    for acl in REQUIRED_ACLS {
        if !acls.contains(&acl.to_string()) {
            return Err(anyhow!("Token missing {} acl", acl));
        }
    }

    Ok(())
}
