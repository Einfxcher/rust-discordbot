use crate::{Context, Error, utils};
use std::time::Instant;

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let latency = start.elapsed();

    let embed = utils::create_embed(
        "ᴘᴏɴɢ",
        "",
        &format!("ʟᴀᴛᴇɴᴄʏ: **{:.3}ᴍѕ**", latency.as_millis()),
        "",
    );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
