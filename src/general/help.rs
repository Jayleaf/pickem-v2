use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage};


pub async fn help(channel_id: &ChannelId, ctx: &Context) {
    let embed = CreateEmbed::new()
        .title("Help")
        .description("Commands:")
        .field("!cg <category>, <home_team>, <away_team>", "Create a game", false)
        .field("!dg <game_id>", "Display a game", false)
        .field("!lg <category>", "List games", false)
        .field("!eval <database> <query>", "Evaluate a SQL query (dev only)", false);
    channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await.expect("Failed to send message");
}