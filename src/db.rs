use rusqlite::{params, Connection, Error};
use serde::Serialize;

#[derive(Serialize)]
pub struct GameTransaction {
    game_id: i64,
    user_id: String,
    amount: i64,
    date: String,
}

#[derive(Serialize)]
pub struct GameProfit {
    user_id: String,
    amount: i64,
}

#[derive(Serialize)]
pub struct GameProfitByDay {
    user_id: String,
    date: String,
    amount: i64,
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut db = Db {
            conn: Connection::open(path)?,
        };
        db.initialize()?;
        Ok(db)
    }

    pub fn initialize(&mut self) -> Result<(), rusqlite::Error> {
        let tx = self.conn.transaction()?;
        tx.execute("CREATE TABLE IF NOT EXISTS game_info (id INTEGER PRIMARY KEY, start_date TEXT NOT NULL, end_date TEXT NULL, description TEXT NULL)", ())?;
        tx.execute("CREATE TABLE IF NOT EXISTS game_transactions (game_id INTEGER NOT NULL, user_id TEXT NOT NULL, amount INTEGER NOT NULL, date TEXT NOT NULL, FOREIGN KEY (game_id) REFERENCES game_info(id))", ())?;
        tx.commit()?;
        Ok(())
    }

    pub fn close_active_games(&mut self) -> Result<(), rusqlite::Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "UPDATE game_info SET end_date = datetime() WHERE end_date IS NULL",
            (),
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn new_game(&mut self, description: &str) -> Result<i64, rusqlite::Error> {
        let tx = self.conn.transaction()?;
        tx.execute("INSERT INTO game_info (start_date, end_date, description) VALUES (datetime(), NULL, ?1)", [description])?;
        tx.execute_batch("COMMIT")?;
        let id = tx.last_insert_rowid();
        Ok(id)
    }

    pub fn save_transaction(
        &mut self,
        game_id: i64,
        user_id: &str,
        amount: i64,
    ) -> Result<(), rusqlite::Error> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO game_transactions (game_id, user_id, amount, date) VALUES (?1, ?2, ?3, datetime())",
            params![game_id, user_id, amount],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn get_all_transactions(&mut self) -> Result<Vec<GameTransaction>, rusqlite::Error> {
        let tx = self.conn.transaction()?;
        let mut stmt = tx.prepare(
            "SELECT game_id, user_id, amount, date FROM game_transactions ORDER BY date DESC",
        )?;

        let iter = stmt.query_map([], |row| {
            Ok(GameTransaction {
                game_id: row.get(0)?,
                user_id: row.get(1)?,
                amount: row.get(2)?,
                date: row.get(3)?,
            })
        })?;

        iter.collect()
    }

    pub fn get_profit(&mut self) -> Result<Vec<GameProfit>, rusqlite::Error> {
        let tx = self.conn.transaction()?;
        let mut stmt = tx.prepare(
            "SELECT user_id, SUM(amount) AS sum FROM game_transactions gt GROUP BY user_id ORDER BY user_id",
        )?;

        let iter = stmt.query_map([], |row| {
            Ok(GameProfit {
                user_id: row.get(0)?,
                amount: row.get(1)?,
            })
        })?;

        iter.collect()
    }

    pub fn get_profit_by_day(&mut self) -> Result<Vec<GameProfitByDay>, rusqlite::Error> {
        let tx = self.conn.transaction()?;
        let mut stmt = tx.prepare(
            "SELECT user_id, date(\"date\") AS date, sum(amount) AS total FROM game_transactions gt GROUP BY gt.user_id, date(gt.\"date\") ORDER BY gt.date DESC",
        )?;

        let iter = stmt.query_map([], |row| {
            Ok(GameProfitByDay {
                user_id: row.get(0)?,
                date: row.get(1)?,
                amount: row.get(2)?,
            })
        })?;

        iter.collect()
    }
}
