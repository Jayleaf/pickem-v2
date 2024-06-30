
use rusqlite::params;
use serenity::all::{AnswerId, ChannelId, Context, MessageId};

use crate::db::structs::{Categories, Game, Record};

pub async fn decide(ctx: Context, channel_id: &ChannelId, content: &str) {
    // Proper Formatting:
    // <game_id> <team>
    // Ex: !dcg 156655da-e5ed-43ba-b2bf-ab560b699c13 Packers

    // Logging format:
    // (<event title>: <win/loss/tie>)
    // Ex: (Packers v. Bears: WIN)
    let content = content.replace("!dg", "");
    let content = content.trim().split(" ").collect::<Vec<_>>();
    let game_id = content[0].trim();
    let team = content[1].trim();

    let connection = rusqlite::Connection::open("games.db").expect("Failed to open database");
    let game = connection.query_row("SELECT * FROM games WHERE id = ?1", params![game_id], |row| {
        Ok(Game {
            id: row.get(0)?,
            category: {
                match row.get::<_, String>(1)?.as_str() { // Add type annotation for the `get` method
                    "NFL" => Categories::NFL,
                    "NBA" => Categories::NBA,
                    "UFC" => Categories::UFC,
                    _ => panic!("Invalid category") // improper
                }
            },
            home_team: row.get(2)?,
            away_team: row.get(3)?,
            winner: row.get(4)?,
            message_id: row.get(5)?,
        })
    });
    let Ok(game) = game else { channel_id.say(&ctx.http, "Invalid Game ID").await.expect("Failed to send message..."); return; };
    if team != game.home_team && team != game.away_team {
        channel_id.say(&ctx.http, "Invalid team").await.expect("Failed to send message...");
        return;
    }
    connection.execute("UPDATE games SET winner = ?1 WHERE id = ?2", params![team, game_id]).expect("Failed to update winner");
    let message_id = MessageId::from(game.message_id.parse::<u64>().expect("Failed to parse message id"));
    let message = channel_id.message(&ctx.http, message_id).await.expect("Failed to get message");
    let poll = message.poll.unwrap();
    let home_voters = 
    channel_id.get_poll_answer_voters(
        &ctx.http, 
        message_id, 
        poll.answers[0].answer_id, 
        None, 
        None
    ).await.unwrap();
    let away_voters =
    channel_id.get_poll_answer_voters(
        &ctx.http, 
        message_id, 
        poll.answers[1].answer_id, 
        None, 
        None
    ).await.unwrap();
    let connection = rusqlite::Connection::open("records.db").expect("Failed to open database");
    let winning_team = if team == game.home_team { &home_voters } else { &away_voters };
    let losing_team = if team == game.home_team { &away_voters } else { &home_voters };
    for user in winning_team {
        let user_record: Option<Record> = {
            println!("{:?}", user.id.to_string());
            println!("{:?}", game.category.to_string());
            let record = connection.query_row("SELECT * FROM records WHERE id = ?1 AND category = ?2", params![user.id.to_string(), game.category.to_string()], |row| {
                Ok(Record {
                    id: row.get(0)?,
                    category: Categories::from_string(row.get::<_, String>(1)?.as_str()),
                    wins: row.get(2)?,
                    losses: row.get(3)?,
                    ties: row.get(4)?,
                    log: row.get(5)?,
                })
            });
            println!("{:?}", record);
            if record.is_err() {
                None
            } else {
                Some(record.unwrap())
            }
        };
        match user_record.clone() {
            Some(mut record) => {
                record.wins += 1;
                connection.execute("UPDATE records SET wins = ?1 WHERE id = ?2 AND category = ?3", params![record.wins, record.id, game.category.to_string()]).expect("Failed to update record");
                let mut win_log = user_record.clone().unwrap().log.split("||").map(|x| x.to_string()).collect::<Vec<String>>();
                win_log.push(format!("({} v. {}: WIN)", game.home_team, game.away_team));
                let win_log = win_log.join("||");
                connection.execute("UPDATE records SET log = ?1 WHERE id = ?2 AND category = ?3", params![win_log, &user_record.unwrap().id, game.category.to_string()]).expect("Failed to update record");
            },
            None => {
                connection.execute("INSERT INTO records (id, category, wins, losses, ties, log) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![user.id.to_string(), game.category.to_string(), 1, 0, 0, format!("({} v. {}: WIN)", game.home_team, game.away_team)]).expect("Failed to insert record");
            }
        }


        

        
    }
    for user in losing_team {
        let user_record: Option<Record> = {
            let record = connection.query_row("SELECT * FROM records WHERE id = ?1 AND category = ?2", params![user.id.to_string(), game.category.to_string()], |row| {
                Ok(Record {
                    id: row.get(0)?,
                    category: Categories::from_string(row.get::<_, String>(1)?.as_str()),
                    wins: row.get(2)?,
                    losses: row.get(3)?,
                    ties: row.get(4)?,
                    log: row.get(5)?,
                })
            });
            if record.is_err() {
                None
            } else {
                Some(record.unwrap())
            }
        };
        match user_record.clone() {
            Some(mut record) => {
                record.losses += 1;
                connection.execute("UPDATE records SET losses = ?1 WHERE id = ?2 AND category = ?3", params![record.losses, record.id, game.category.to_string()]).expect("Failed to update record");
                let mut loss_log = user_record.clone().unwrap().log.split("||").map(|x| x.to_string()).collect::<Vec<String>>();
                loss_log.push(format!("({} v. {}: LOSS)", game.home_team, game.away_team));
                let loss_log = loss_log.join("||");
                connection.execute("UPDATE records SET log = ?1 WHERE id = ?2 AND category = ?3", params![loss_log, &user_record.unwrap().id, game.category.to_string()]).expect("Failed to update record");
            },
            None => {
                connection.execute("INSERT INTO records (id, category, wins, losses, ties, log) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![user.id.to_string(), game.category.to_string(), 0, 1, 0, format!("({} v. {}: LOSS)", game.home_team, game.away_team)]).expect("Failed to insert record");
            }
        };
       

    }
}