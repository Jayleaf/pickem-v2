use std::fs::File;

use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::db::structs;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub games: Vec<String>,
}

pub async fn create_bulk(ctx: Context, msg: Message) {
    // Proper Formatting:
    // <category>, <set>
    // Ex: !crgb NFL week_1_reg

    let content = msg.content.replace("!crgb", "");
    let content = content.trim().split(" ").collect::<Vec<_>>();
    println!("{:?}", content);
    let category = {
        match content[0].trim() {
            "NFL" => structs::Categories::NFL,
            "NBA" => structs::Categories::NBA,
            "UFC" => structs::Categories::UFC,
            _ => { 
                msg.channel_id.say(&ctx.http, "Invalid Category (NFL and NBA supported)")
                .await
                .expect("Failed to send message..."); 
                return
            }
        }
    };
    let set = content[1].trim();

    let path = format!("presets/{}/{}.json", category.to_string(), set);
    let Ok(file) = File::open(path)
    else { msg.channel_id.say(&ctx.http, "invalid category or set (couldn't find preset file)").await.expect("Failed to send message..."); return; };
    let mut parsed_games = vec![];
    let games: Root = serde_json::from_reader(file).unwrap();
    for g in games.games { // would love to make this functional but i dont wanna deal with async move closures
        let teams = g.split(" at ").collect::<Vec<&str>>();
        let home_team = teams[1];
        let away_team = teams[0];

        let Ok(game)= structs::Game::new(category, home_team.parse().unwrap(), away_team.parse().unwrap()).await
        else { msg.channel_id.say(&ctx.http, format!("Failed to create event")).await.expect("Failed to send message..."); return; };
        parsed_games.push(game);
    };
    msg.channel_id.say(&ctx.http, format!("success")).await.expect("Failed to send message...");
    
}