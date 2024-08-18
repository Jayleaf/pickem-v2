use rusqlite::Connection;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum Categories {
    NFL,
    NBA,
    UFC,
}

impl Categories {
    pub fn to_string(&self) -> String {
        match self {
            Categories::NFL => "NFL".to_string(),
            Categories::NBA => "NBA".to_string(),
            Categories::UFC => "UFC".to_string(),
        }
    }
    pub fn from_string(str: &str) -> Categories {
        match str {
            "NFL" => Categories::NFL,
            "NBA" => Categories::NBA,
            "UFC" => Categories::UFC,
            _ => panic!("Invalid category"),
        }
    }

}

#[derive(Debug, Clone)]

pub struct Record {
    pub id: String,
    pub category: Categories,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub log: String,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String,
    pub category: Categories,
    pub home_team: String,
    pub away_team: String,
    pub winner: String,
    pub message_id: String,
}


impl Game {

    pub async fn new(category: Categories, home_team: String, away_team: String) -> Result<Game, rusqlite::Error> {
        let connection = Connection::open("games.db")?;
        let game = Game {
            id: Uuid::new_v4().to_string(),
            category,
            home_team,
            away_team,
            winner: "UNDECIDED".to_string(),
            message_id: String::new(),
        };

        connection.execute("
        INSERT INTO games VALUES (?1, ?2, ?3, ?4, ?5, ?6) ",
        (
            &game.id,                   // ?1
            &game.category.to_string(), // ?2
            &game.home_team,            // ?3
            &game.away_team,            // ?4
            &game.winner,               // ?5
            &game.message_id,           // ?6
        ))?;
        Ok(game)
    }
}