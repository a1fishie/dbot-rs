use crate::cmds::misc::get_user;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, prelude::*, user::User},
    prelude::*,
    utils::Colour,
};

#[command]
pub async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg
        .guild_id
        .expect("Guild ID not found")
        .to_partial_guild(&ctx.http)
        .await?;
    let id = match args.current() {
        Some(id) => id,
        None => {
            msg.channel_id
                .say(&ctx.http, "HELP! Provide user id")
                .await?;
            panic!("Could not retrive user")
        }
    };
    let user = get_user(id, ctx).await?;
    let dmd = match args.advance().single::<u8>() {
        Ok(dmd) => dmd,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "HELP! Provide number of days to purge user messages",
                )
                .await?;
            panic!("Could not retrive days")
        }
    };
    let reason = match args.remains() {
        Some(reason) => reason,
        None => {
            msg.channel_id
                .say(&ctx.http, "HELP! Provide the reason for ban")
                .await?;
            panic!("Could not retrive ban reason")
        }
    };
    guild.ban_with_reason(&ctx.http, &user, dmd, reason).await?;
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Banned: ").fields(vec![
                    ("User: ", user.id.to_string(), true),
                    ("Reason: ", reason.to_string(), true),
                ])
                    .color(Colour::RED)
            })
        })
        .await?;
    Ok(())
}

#[command]
pub async fn unban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg
        .guild_id
        .expect("Guild ID not found")
        .to_partial_guild(&ctx.http)
        .await?;
    let id = match args.current() {
        Some(id) => id,
        None => {
            msg.channel_id
                .say(&ctx.http, "HELP! Provide the user to unban")
                .await?;
            "NULL"
        }
    };
    let user_id = get_user(id, ctx).await.unwrap();
    guild.unban(&ctx.http, &user_id).await?;
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.title("Unbanned: ").field("User: ", user_id, true).color(Colour::RED))
        })
        .await?;
    Ok(())
}

#[command]
pub async fn clear(ctx: &Context, msg: &Message, mut arg: Args) -> CommandResult {
    let del_count = match arg.single::<u64>() {
        Ok(msg) => msg * 2,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "HELP! Provide number of messages to clear")
                .await?;
            0
        }
    };
    let messages = msg
        .channel_id
        .messages(&ctx.http, |retriver| {
            retriver.around(MessageId(msg.id.0)).limit(del_count)
        })
        .await
        .unwrap();
    if del_count > 0 {
        msg.channel_id.delete_messages(&ctx.http, messages).await?;
        msg.channel_id
            .say(&ctx.http, format!("Cleared {} messages", del_count / 2))
            .await?;
        println!("Cleared {} messages", del_count / 2);
    }
    Ok(())
}

#[command]
pub async fn rolemenu(ctx: &Context, msg: &Message, arg: Args) -> CommandResult {
    match arg.message() {
        "new" => new_rolemenu(ctx, msg),
        "remove" => new_rolemenu(ctx, msg),
        "complete" => new_rolemenu(ctx, msg),
    };
    Ok(())
}

pub async fn new_rolemenu(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Provide name of role to add").await?;
    if msg.author.await_reply().await? ;
    Ok(())
}
