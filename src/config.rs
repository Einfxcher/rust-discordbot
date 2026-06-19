use anyhow::{Result, anyhow};

pub fn get_token() -> Result<String> {
    std::env::var("DISCORD_TOKEN").map_err(|_| {
        anyhow!("DISCORD_TOKEN environment variable not set. Create a .env file with your token.")
    })
}

pub fn get_honeypot_channel() -> Result<u64> {
    std::env::var("HONEYPOT_CHANNEL_ID")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| anyhow!("HONEYPOT_CHANNEL_ID not set in .env"))
}

pub fn get_log_channel() -> Result<u64> {
    std::env::var("LOG_CHANNEL_ID")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| anyhow!("LOG_CHANNEL_ID not set in .env"))
}
