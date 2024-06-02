use crate::db::structs::{Categories, Record};
use rusqlite::params;
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, UserId};

pub async fn record(ctx: &Context, channel_id: ChannelId, user: UserId) {
    let records: Result<Vec<Record>, rusqlite::Error> = tokio::task::spawn_blocking({
        move || -> Result<Vec<Record>, rusqlite::Error> {
            let mut records: Vec<Record> = Vec::new();
            let connection = rusqlite::Connection::open("records.db").expect("Failed to open database");
            let mut statement = connection.prepare(&format!("SELECT * FROM records WHERE id = {x}", x = user.to_string()))?;
            
            let mut rows = statement.query([]).unwrap();
            while let Ok(Some(row)) = rows.next() {
                let id: String = row.get(0).expect("Failed to get id");
                let category = {
                    match row.get::<_, String>(1)?.as_str() { // Add type annotation for the `get` method
                        "NFL" => Categories::NFL,
                        "NBA" => Categories::NBA,
                        "UFC" => Categories::UFC,
                        _ => panic!("Invalid category") // improper
                    }
                };
                let wins: i32 = row.get(2).expect("Failed to get wins");
                let losses: i32 = row.get(3).expect("Failed to get losses");
                let ties: i32 = row.get(4).expect("Failed to get ties");
                let log: String = row.get(5).expect("Failed to get log");

                records.push(Record {
                    id,
                    category,
                    wins,
                    losses,
                    ties,
                    log,
                });
            }
            Ok(records)
        }
    }).await.expect("Failed to get games");
    let Ok(records) = records.as_ref() else { channel_id.say(&ctx.http, "Failed to get records").await.expect("Failed to send message"); return; };
    let mut embed = CreateEmbed::new()
        .title(format!("Records for {x}", x = user.to_string()));
    let mut log_field: Vec<String> = Vec::new();
    for record in records {
        embed = embed.field(format!("{x}", x = record.category.to_string()), format!("{x} Wins | {y} Losses | {z} Ties", x = record.wins, y = record.losses, z = record.ties), false);
        log_field.push(record.log.split("||").collect::<Vec<&str>>().join("\n"));
    }
    let message = CreateMessage::new().embed(embed);
    let _ = channel_id.send_message(&ctx.http, message).await;
}