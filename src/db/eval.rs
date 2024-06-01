use rusqlite::Connection;
use serenity::all::{ChannelId, Context};
pub async fn eval(msg: &str, channel_id: &ChannelId, ctx: &Context) {
    // very dangerous, only for testing
    let db = {
        match msg.split(" ").collect::<Vec<_>>()[0] {
            "games" => "games.db",
            "records" => "records.db",
            _ => {
                channel_id.say(&ctx.http, "Invalid database").await.expect("Failed to send message");
                return;
            }
        }
    };
    let query = msg.replacen(&format!("{x}", x=db.replace(".db", "")), "", 1).trim().to_string();
    println!("{db}");
    let connection = Connection::open(db).expect("Failed to open database");
    let res = connection.execute(&query, []);
    match res {
        Ok(_) => channel_id.say(&ctx.http, "Executed successfully").await.expect("Failed to send message"),
        Err(e) => channel_id.say(&ctx.http, format!("Error: {:?}", e)).await.expect("Failed to send message"),
    };
}
