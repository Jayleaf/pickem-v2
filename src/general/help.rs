use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage};


pub async fn help(channel_id: &ChannelId, ctx: &Context) {
    let embed = CreateEmbed::new()
        .title("Help")
        .description("Commands:")
        .field("!crg <category> <home_team> <away_team>", "Create a game", false)
        .field("!dsg <game_id>", "Display a game", false)
        .field("!dcg <game_id> <team>", "Decide a game's winner", false)
        .field("!lg <category>", "List active games of a category", false)
        .field("!clg <game_id>", "Close a game's poll", false)
        .field("!r", "Display your records", false)

        .field("!eval <database> <query>", "Evaluate a SQL query (dev only)", false);
    channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await.expect("Failed to send message");
}