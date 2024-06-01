
pub async fn close(ctx: serenity::client::Context, channel_id: serenity::model::id::ChannelId, content: &str) {
    // Proper Formatting:
    // <id>
    // Ex: !clg adc1dcd9-b8f0-49bc-bd25-13c86fd3df30

    let game = content.trim();
    let db = rusqlite::Connection::open("games.db").expect("Failed to open database");
    let game_record = db.query_row("SELECT * FROM games WHERE id = ?1", rusqlite::params![game], |row| {
        Ok(crate::db::structs::Game {
            id: row.get(0)?,
            category: {
                match row.get::<_, String>(1)?.as_str() {
                    "NFL" => crate::db::structs::Categories::NFL,
                    "NBA" => crate::db::structs::Categories::NBA,
                    _ => panic!("Invalid category")
                }
            },
            home_team: row.get(2)?,
            away_team: row.get(3)?,
            winner: row.get(4)?,
            message_id: row.get(5)?,
        })
    });
    let game_record = match game_record {
        Ok(game) => game,
        Err(_) => { channel_id.say(&ctx.http, "Invalid Game ID").await.expect("Failed to send message..."); return; }
    };
    let message_id = serenity::model::id::MessageId::from(game_record.message_id.parse::<u64>().expect("Failed to parse message id"));
    let message = channel_id.message(&ctx.http, message_id).await.expect("Failed to get message");
    message.end_poll(&ctx.http).await.expect("Failed to end poll");
    
}