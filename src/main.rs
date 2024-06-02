mod db;
mod games;
mod general;
mod user;

use db::eval;
use serenity::model::channel::Message;
use serenity::async_trait;
use serenity::prelude::*;


struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let command = msg.content.split_whitespace().collect::<Vec<_>>()[0];
        let content = msg.content.replace(command, "").trim().to_string();
        if msg.author.id != 395205580668534785 && !content.starts_with("!r") {
            return;
        }
        match command {
            "!ping" => {
                if let Err(x) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {:?}", x);
                }
            }
            "!help" => {
                general::help::help(&msg.channel_id, &ctx).await;
            }
            "!eval" => {
            
                eval::eval(&content, &msg.channel_id, &ctx).await;
            }
            "!crg" => {
                games::create::create(ctx, msg).await;
            }
            "!dsg" => {
                games::display::display(msg.channel_id, ctx, &content).await;
            }
            "!dcg" => {
                games::decide::decide(ctx, &msg.channel_id, &content).await;
            }
            "!lg" => {
                games::list::list(ctx, msg.channel_id, &content).await;
            }
            "!clg" => {
                games::close::close(ctx, msg.channel_id, &content).await;
            }
            "!r" => {
                user::record::record(&ctx, msg.channel_id, msg.author.id).await;
            }

            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv::var("TOKEN").expect("Expected a token in the environment");
    let intents =
        GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}")
    }
}
