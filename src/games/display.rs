use serenity::{all::ChannelId, builder::{CreateMessage, CreatePoll, CreatePollAnswer}, model::connection};
use rusqlite::{params, Connection};
use serenity::prelude::*;

use crate::db::structs::{Categories, Game};
pub async fn display(channel_id: ChannelId, ctx: Context, game_id: &str) {
    /* 
    ⚠️ WARNING:
    ⚠️ Running this function will create a NEW poll.

    Proper Formatting:
    <game_id>
    Ex: !dsg 156655da-e5ed-43ba-b2bf-ab560b699c13
    */
    let connection = Connection::open("games.db").expect("Failed to open database");
    let game = connection.query_row("SELECT * FROM games WHERE id = ?1", params![game_id], |row| {
        Ok(Game {
            id: row.get(0)?,
            category: {
                match row.get::<_, String>(1)?.as_str() { // Add type annotation for the `get` method
                    "NFL" => Categories::NFL,
                    "NBA" => Categories::NBA,
                    _ => panic!("Invalid category") // improper
                }
            },
            home_team: row.get(2)?,
            away_team: row.get(3)?,
            winner: row.get(4)?,
            message_id: row.get(5)?,
        })

    }).expect("Failed to get game");
    
    let connection = Connection::open("teams.db").expect("Failed to open database");
    let home_team_emoji = connection.query_row("SELECT emoji FROM teams WHERE name = ?1", params![game.home_team], |row| {
        Ok(row.get::<_, String>(0)?.to_string())
    }).expect("Failed to get home team emoji");
    let away_team_emoji = connection.query_row("SELECT emoji FROM teams WHERE name = ?1", params![game.away_team], |row| {
        Ok(row.get::<_, String>(0)?.to_string())
    }).expect("Failed to get away team emoji");
    let poll = CreatePoll::new()
        .question(format!("{y} at {x} || {z}", x = game.home_team, y = game.away_team, z = game.category.to_string()))
        .answers(vec![
            CreatePollAnswer::new().emoji(home_team_emoji).text(format!("{x}", x=game.home_team)),
            CreatePollAnswer::new().emoji(away_team_emoji).text(format!("{y}", y=game.away_team)),
        ])
        .duration(std::time::Duration::from_secs(60 * 60 * 24 * 7));
    let message = CreateMessage::new().poll(poll);
    let message = channel_id.send_message(&ctx.http, message).await.expect("Failed to send message");
    let connection = Connection::open("games.db").expect("Failed to open database");
    connection.execute("UPDATE games SET message_id = ?1 WHERE id = ?2", params![message.id.to_string(), game_id]).expect("Failed to update message id");
}