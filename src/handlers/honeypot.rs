use crate::{config, utils};
use anyhow::anyhow;
use chrono::Duration;
use poise::serenity_prelude as serenity;

pub async fn handle_honeypot_message(
    ctx: &serenity::Context,
    msg: &serenity::Message,
) -> anyhow::Result<()> {
    let honeypot_id = config::get_honeypot_channel()?;
    let log_id = config::get_log_channel()?;

    // Check if message is in honeypot channel
    if msg.channel_id.get() != honeypot_id {
        return Ok(());
    }

    // Don't act on bot messages
    if msg.author.bot {
        return Ok(());
    }

    let guild_id = msg.guild_id.ok_or(anyhow!("No guild ID"))?;
    let mut member = guild_id.member(ctx, msg.author.id).await?;

    // Calculate timeout until (24 hours from now)
    let timeout_until = chrono::Utc::now() + Duration::hours(24);
    member
        .disable_communication_until_datetime(ctx, timeout_until.into())
        .await?;

    // Build description with content and attachments
    let mut description = format!(
        "**User:** {} ({})\n**Message:** {}\n**Timeout:** 24 hours",
        msg.author.tag(),
        msg.author.id,
        if msg.content.is_empty() {
            "(no text)".to_string()
        } else {
            msg.content.clone()
        }
    );

    // Add attachment info if there are any
    if !msg.attachments.is_empty() {
        description.push_str("\n\n**Attachments:**\n");
        for attachment in &msg.attachments {
            description.push_str(&format!(
                "[{}]({})\n",
                attachment.filename, attachment.proxy_url
            ));
        }
    }

    // Create log embed
    let mut log_embed = utils::create_embed(
        "Honeypot Alert",
        "🍯",
        &description,
        &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    );

    // If there's an image attachment, set it as the embed image
    if let Some(attachment) = msg.attachments.first() {
        if attachment
            .content_type
            .as_ref()
            .map_or(false, |ct| ct.starts_with("image/"))
        {
            log_embed = log_embed.image(&attachment.proxy_url);
        }
    }

    // Send to log channel
    let log_channel = serenity::ChannelId::new(log_id);
    log_channel
        .send_message(ctx, serenity::CreateMessage::default().embed(log_embed))
        .await?;

    // Delete all messages from this user in the last 10 minutes across ALL channels
    let ten_minutes_ago = chrono::Utc::now() - Duration::minutes(10);
    let guild = guild_id.to_partial_guild(ctx).await?;
    let mut deleted_count = 0;

    for channel_id in guild.channels(ctx).await?.keys() {
        // Skip non-text channels
        if let Ok(messages) = channel_id
            .messages(ctx, serenity::GetMessages::new().limit(100))
            .await
        {
            for message in messages {
                if message.author.id == msg.author.id
                    && message.timestamp.timestamp() > ten_minutes_ago.timestamp()
                {
                    message.delete(ctx).await.ok();
                    deleted_count += 1;
                }
            }
        }
    }

    tracing::warn!(
        "🍯 Honeypot triggered by {} ({}). Deleted {} recent messages from all channels.",
        msg.author.tag(),
        msg.author.id,
        deleted_count
    );

    Ok(())
}
