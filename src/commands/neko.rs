use super::super::poise_data::*;
use poise::serenity_prelude::{self as serenity, Mentionable};


/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn neko(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.say(format!("{} にゃーん", ctx.author().mention())).await?;
    Ok(())
}
