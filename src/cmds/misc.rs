use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::{client::Http, CacheHttp},
    model::{channel::Message, user::User, Timestamp},
    prelude::*,
    utils::{Colour, parse_username},
};
use rand::prelude::*;

pub async fn get_user(query: &str, ctx: &Context) -> Result<User, SerenityError> {
    if query.contains("<@") {
        let id = parse_username(query).ok_or("Could not parse username");
        Http::get_user(&ctx.http, id.unwrap()).await
    } else {
        Http::get_user(&ctx.http, query.parse::<u64>().unwrap()).await
    }
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    println!("Ponged!");
    Ok(())
}

#[command]
pub async fn tias(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "https://tryitands.ee/")
        .await?;
    println!("Ran tias");
    Ok(())
}

#[command]
pub async fn whois(ctx: &Context, msg: &Message, arg: Args) -> CommandResult {
    if arg.is_empty() {
        msg.channel_id
            .say(&ctx.http, "HELP! Provide user to search")
            .await?;
    }
    let query = arg.current().unwrap();
    let user = get_user(query, ctx).await.unwrap();
    let pfp_url = User::static_avatar_url(&user).unwrap();
    let tag = User::tag(&user);
    let created_at = user.created_at().to_string();
    let nick = match User::nick_in(&user, &ctx.http.http(), msg.guild_id.unwrap()).await {
        Some(nick) => nick,
        None => user.name,
    };
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&tag)
                    .thumbnail(pfp_url)
                    .fields(vec![
                        ("Username: ", nick, true),
                        ("ID: ", user.id.as_u64().to_string(), true),
                        ("Created at: ", created_at, false),
                    ])
                    .timestamp(Timestamp::now())
            })
        })
        .await?;
    Ok(())
}

#[command]
pub async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    let x = rand::thread_rng().gen_range(1..=6);
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Dice has been rolled")
                    .field("Result is: ", format!("{x}"), true)
                    .timestamp(Timestamp::now())
                    .colour(Colour::BLUE)
            })
        }).await?;
    Ok(())
}
