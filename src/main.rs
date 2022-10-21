mod cmds;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::{
    async_trait, client::bridge::gateway::ShardManager, framework::standard::macros::group,
    framework::StandardFramework, http::Http, model::event::ResumedEvent, model::gateway::Ready,
    prelude::*,
};
use tracing::{error, info};

use crate::cmds::admin::{BAN_COMMAND, CLEAR_COMMAND, UNBAN_COMMAND};
use crate::cmds::misc::{PING_COMMAND, ROLL_COMMAND, TIAS_COMMAND, WHOIS_COMMAND};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping, tias, whois, roll, unban, ban, clear)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("where token man");
    let http = Http::new(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access app info {:?}", why),
    };
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP);
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Failed in creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c hadnler");
        shard_manager.lock().await.shutdown_all().await
    });

    if let Err(why) = client.start().await {
        error!("Client error {:?}", why);
    }
}
