use poise::serenity_prelude as serenity;

pub fn create_embed(
    title: &str,
    emoji: &str,
    description: &str,
    footer: &str,
) -> serenity::CreateEmbed {
    serenity::CreateEmbed::default()
        .title(format!("{} {}", emoji, title))
        .description(description)
        .footer(serenity::CreateEmbedFooter::new(footer))
        .color(serenity::Color::from_rgb(0x3A, 0x3A, 0x3C))
}
