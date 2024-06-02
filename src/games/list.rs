
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage};
use crate::db::structs::Game;

pub async fn list(ctx: Context, channel_id: ChannelId, msg: &str) {
    // Proper Formatting:
    // <category>
    // Ex: !lg NFL
    let content = msg.replace("!lg", "");
    let content = content.trim();
    let category = {
        match content {
            "NFL" => crate::db::structs::Categories::NFL,
            "NBA" => crate::db::structs::Categories::NBA,
            "UFC" => crate::db::structs::Categories::UFC,
            _ => { 
                return
            }
        }
    };
    let games: Result<Vec<Game>, rusqlite::Error> = tokio::task::spawn_blocking({
        let category = category.clone(); 

        move || -> Result<Vec<Game>, rusqlite::Error> {
            let mut games: Vec<Game> = Vec::new();
            let connection = rusqlite::Connection::open("games.db").expect("Failed to open database");
            let mut statement = connection.prepare(&format!("SELECT * FROM games WHERE category = \"{x}\"", x = category.to_string()))?;
            
            let mut rows = statement.query([]).unwrap();
            while let Ok(Some(row)) = rows.next() {
                let id: String = row.get(0).expect("Failed to get id");
                let category: String = row.get(1).expect("Failed to get category");
                let home_team: String = row.get(2).expect("Failed to get home team");
                let away_team: String = row.get(3).expect("Failed to get away team");
                games.push(Game {
                    id,
                    category: {
                        match category.as_str() {
                            "NFL" => crate::db::structs::Categories::NFL,
                            "NBA" => crate::db::structs::Categories::NBA,
                            "UFC" => crate::db::structs::Categories::UFC,
                            _ => panic!("Invalid category")
                        }
                    },
                    home_team,
                    away_team,
                    winner: row.get(4).expect("Failed to get winner"),
                    message_id: row.get(5).expect("Failed to get message id"),
                });
            }
            Ok(games)
        }
    }).await.expect("Failed to get games");
    if let Err(x) = games.as_ref() { channel_id.say(&ctx.http, format!("Failed to get games: {:?}", x)).await.expect("Failed to send message"); }
    else if games.as_ref().unwrap().is_empty() { channel_id.say(&ctx.http, "No games found.").await.expect("Failed to send message"); }
    let games = games.unwrap();
    let mut embed = CreateEmbed::new()
        .title(format!("Category: {x} | Count: {y}", x = category.to_string(), y = games.len()));
    for game in games {
        embed = embed.field(format!("{x}", x = game.id), format!("{x} v. {y} | Winner: {z}", x = game.home_team, y = game.away_team, z = game.winner), false);
    }
    let message = CreateMessage::new().embed(embed);
    let _ = channel_id.send_message(&ctx.http, message).await;
    

    
    
}