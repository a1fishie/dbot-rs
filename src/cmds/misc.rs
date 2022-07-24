use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::model::prelude::*;
use serenity::model::Timestamp;
use serenity::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[command]
pub async fn tias(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "https://tryitands.ee/").await?;
    Ok(())
}

#[command]
pub async fn whois(ctx: &Context, msg: &Message, arg: Args) -> CommandResult {
    let query: u64 = arg.current().unwrap().to_owned().parse::<u64>().unwrap();
    let user = Http::get_user(&ctx.http, query).await?;
    let pfp_url = User::avatar_url(&user);
    let tag = User::tag(&user);
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(tag)
                    .fields(vec![
                            ("Avatar url", pfp_url.unwrap(), true),
                    ])
                    .footer(|f| f.text("Called: "))
                    .timestamp(Timestamp::now())
            })
        })
        .await?;
    Ok(())
    // pfp, created at, nicknames, msg count, roles?
}


#[command]
pub async fn amogus(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("sus").field("https://www.youtube.com/watch?v=5DlROhT8NgU", "heavy sussy bass", false).footer(|f| f.text("a m o g s u s"))
        })
    }).await?;
    Ok(())
}

