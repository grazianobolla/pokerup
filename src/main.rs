#[macro_use]
extern crate rouille;

mod db;
mod game;
mod server;

fn main() {
    let database = db::Db::new("database.sqlite").expect("Could not create database!");
    let game = std::sync::Mutex::new(game::Game::new(database));
    server::start_server(game, "localhost:25563");
}
