use serenity::{all::ChannelId, builder::{CreateMessage, CreatePoll, CreatePollAnswer}};
use rusqlite::{params, Connection};
use serenity::prelude::*;

use crate::db::structs::{Categories, Game};
pub async fn display_bulk(channel_id: ChannelId, ctx: Context, category: &str) {
    /* 
    Displays all games in a category.

    Proper Formatting:
    <game_id>
    Ex: !dsg 156655da-e5ed-43ba-b2bf-ab560b699c13
    */
    let mut games: Vec<Game> = vec![];
    { // compiler gets real mad if i don't make a new scope for this block
        let connection = Connection::open("games.db").expect("Failed to open database");
        let mut statement = connection.prepare(&format!("SELECT * FROM games WHERE category = \"{x}\" AND winner = \"UNDECIDED\"", x = category)).unwrap();

        let mut rows = statement.query([]).unwrap();
        while let Ok(Some(row)) = rows.next() {
            let game = Game {
                id: row.get(0).unwrap(),
                category: Categories::from_string(category),
                home_team: row.get(2).unwrap(),
                away_team: row.get(3).unwrap(),
                winner: row.get(4).unwrap(),
                message_id: row.get(5).unwrap(),
            };
            games.push(game)
        }
    }
    
    let connection = Connection::open("teams.db").expect("Failed to open database");
    for game in games {
        println!("{}", game.home_team);
        let home_team_emoji =  {
            if game.category.to_string() != "UFC" {
                connection.query_row("SELECT emoji FROM teams WHERE name = ?1", params![game.home_team], |row| {
                    Ok(row.get::<_, String>(0)?.to_string())
                }).expect("Failed to get home team emoji")
            } else { "🔴".to_string() }
        };
        let away_team_emoji = {
            if game.category.to_string() != "UFC" {
                connection.query_row("SELECT emoji FROM teams WHERE name = ?1", params![game.away_team], |row| {
                    Ok(row.get::<_, String>(0)?.to_string())
                }).expect("Failed to get away team emoji")
            } else { "🔵".to_string() }
        };
        let poll = CreatePoll::new()
            .question(format!("{y} v. {x} || {z}", x = game.home_team, y = game.away_team, z = game.category.to_string()))
            .answers(vec![
                CreatePollAnswer::new().emoji(home_team_emoji).text(format!("{x}", x=game.home_team)),
                CreatePollAnswer::new().emoji(away_team_emoji).text(format!("{y}", y=game.away_team)),
            ])
            .duration(std::time::Duration::from_secs(60 * 60 * 24 * 7));
        
        let message = CreateMessage::new().poll(poll);
        let message = channel_id.send_message(&ctx.http, message).await.expect("Failed to send message");
        let connection = Connection::open("games.db").expect("Failed to open database");
        connection.execute("UPDATE games SET message_id = ?1 WHERE id = ?2", params![message.id.to_string(), game.id]).expect("Failed to update message id");
    }
}