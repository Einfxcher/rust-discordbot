use anyhow::{Result, anyhow};

pub fn get_token() -> Result<String> {
    std::env::var("DISCORD_TOKEN").map_err(|_| {
        anyhow!("DISCORD_TOKEN environment variable not set. Create a .env file with your token.")
    })
}
