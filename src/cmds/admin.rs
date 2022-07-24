use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::model::guild::PartialGuild;
use serenity::model::prelude::*;
use serenity::model::guild;
use serenity::prelude::*;


#[command]
pub async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_query: u64= args.single::<u64>().unwrap();
    let guild = msg.guild_id.expect("Guild ID not found").to_partial_guild(&ctx.http).await?;
    let user = Http::get_user(&ctx.http, user_query).await?;
    let dmd: u8 = args.single::<u8>().unwrap();
    let reason: &str = args.remains().unwrap();
    guild.ban_with_reason(&ctx.http, user.clone(), dmd, reason).await?;
    msg.channel_id.say(&ctx.http, format!("Banned user: {} for: {}", user.id, reason)).await?;
    Ok(())
}

#[command]
pub async fn unban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild_id.expect("Guild ID not found").to_partial_guild(&ctx.http).await?;
    let user_id = args.single::<u64>().unwrap();
    guild.unban(&ctx.http, user_id);
    Ok(())
}
