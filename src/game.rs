use std::error::Error;

use crate::db::{self, GameProfit, GameProfitByDay, GameTransaction};

pub struct Game {
    db: db::Db,
}

impl Game {
    pub fn new(database: db::Db) -> Self {
        Game { db: database }
    }

    pub fn start_game(&mut self, description: &str) -> Result<(), Box<dyn Error>> {
        self.db.close_active_games()?;
        println!("Closing all previous active games!");
        let id: i64 = self.db.new_game(description)?;
        println!("New game {} record created", id);
        Ok(())
    }

    pub fn save_transaction(
        &mut self,
        game_id: i64,
        user_id: &str,
        amount: i64,
    ) -> Result<(), Box<dyn Error>> {
        self.db.save_transaction(game_id, user_id, amount)?;
        println!("User {} performed transaction for ${}", user_id, amount);
        Ok(())
    }

    pub fn get_all_transactions(&mut self) -> Result<Vec<GameTransaction>, Box<dyn Error>> {
        Ok(self.db.get_all_transactions()?)
    }

    pub fn get_profit(&mut self) -> Result<Vec<GameProfit>, Box<dyn Error>> {
        Ok(self.db.get_profit()?)
    }

    pub fn get_profit_by_day(&mut self) -> Result<Vec<GameProfitByDay>, Box<dyn Error>> {
        Ok(self.db.get_profit_by_day()?)
    }
}
