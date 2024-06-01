use rusqlite::params;
use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::db::structs;

pub async fn create(ctx: Context, msg: Message) {
    // Proper Formatting:
    // <category>, <home_team>, <away_team>
    // Ex: !cg NFL Packers Bears

    let content = msg.content.replace("!crg", "");
    let content = content.trim().split(" ").collect::<Vec<_>>();
    println!("{:?}", content);
    let category = {
        match content[0].trim() {
            "NFL" => structs::Categories::NFL,
            "NBA" => structs::Categories::NBA,
            _ => { 
                msg.channel_id.say(&ctx.http, "Invalid Category (NFL and NBA supported)")
                .await
                .expect("Failed to send message..."); 
                return
            }
        }
    };
    let home_team = content[1].trim();
    let away_team = content[2].trim();

    // ensure teams are valid
    let connection = rusqlite::Connection::open("teams.db").expect("Failed to open database");
    let home_team_exists = connection.query_row("SELECT COUNT(*) FROM teams WHERE name = ?1 AND category = ?2", params![home_team, category.to_string()], |row| {
        Ok(row.get::<_, i32>(0)?)
    }).expect("Failed to check if home team exists");
    let away_team_exists = connection.query_row("SELECT COUNT(*) FROM teams WHERE name = ?1 AND category = ?2", params![away_team, category.to_string()], |row| {
        Ok(row.get::<_, i32>(0)?)
    }).expect("Failed to check if away team exists");
    println!("Home Team Exists: {}, Away Team Exists: {}", home_team_exists, away_team_exists);
    if home_team_exists == 0 || away_team_exists == 0 {
        msg.channel_id.say(&ctx.http, "Invalid team(s)").await.expect("Failed to send message...");
        return;
    }

    println!("Creating event: {:?}, {}, {}", category, home_team, away_team);
    let Ok(game)= structs::Game::new(category, home_team.parse().unwrap(), away_team.parse().unwrap()).await
    else { msg.channel_id.say(&ctx.http, format!("Failed to create event")).await.expect("Failed to send message..."); return; };
    
    msg.channel_id.say(&ctx.http, format!("Created event: {:?}", game.id)).await.expect("Failed to send message...");

}