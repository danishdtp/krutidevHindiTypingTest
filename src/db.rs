use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;

use crate::models::Score;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> SqliteResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS scores (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                wpm INTEGER NOT NULL,
                acc INTEGER NOT NULL,
                date TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert_score(&self, wpm: i32, acc: i32, date: &str) -> SqliteResult<()> {
        self.conn.execute(
            "INSERT INTO scores (wpm, acc, date) VALUES (?1, ?2, ?3)",
            rusqlite::params![wpm, acc, date],
        )?;
        Ok(())
    }

    pub fn get_best_wpm(&self) -> SqliteResult<i32> {
        let result = self
            .conn
            .query_row("SELECT MAX(wpm) FROM scores", [], |row| {
                row.get::<_, Option<i32>>(0)
            })?;
        Ok(result.unwrap_or(0))
    }

    pub fn get_all_scores(&self) -> SqliteResult<Vec<Score>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, wpm, acc, date FROM scores ORDER BY id DESC")?;

        let scores = stmt.query_map([], |row| {
            Ok(Score {
                id: Some(row.get(0)?),
                wpm: row.get(1)?,
                acc: row.get(2)?,
                date: row.get(3)?,
            })
        })?;

        scores.collect()
    }
}
