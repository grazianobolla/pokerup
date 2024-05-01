use std::sync::Mutex;

use rouille::Request;

use crate::game::Game;

pub fn start_server(game_mut: Mutex<Game>, address: &str) {
    rouille::start_server(address, move |request| {
        router!(request,
            (POST) (/start_game) => { handle_start_game(&game_mut, request) },
            (POST) (/save_transaction) => { handle_save_transaction(&game_mut, request) },
            (GET) (/get_transactions) => { handle_get_transaction(&game_mut, request) },
            (GET) (/get_profit) => { handle_get_profit(&game_mut, request) },
            (GET) (/get_profit_by_day) => { handle_get_profit_by_day(&game_mut, request) },
            _ => handle_main(request)
        )
    });
}

fn handle_main(request: &Request) -> rouille::Response {
    let res = rouille::match_assets(&request, "public/");

    if res.is_success() {
        return res;
    }

    rouille::Response::empty_404()
}

fn handle_start_game(game_mut: &Mutex<Game>, request: &Request) -> rouille::Response {
    let data = try_or_400!(post_input!(request, {
        description: String,
    }));

    let mut game = game_mut.lock().expect("Error while locking mutex!");
    let res = match game.start_game(&data.description.as_str()) {
        Ok(_) => rouille::Response::text("Ok").with_status_code(200),
        Err(err) => rouille::Response::text(err.to_string()).with_status_code(500),
    };
    return res;
}

fn handle_save_transaction(game_mut: &Mutex<Game>, request: &Request) -> rouille::Response {
    let data = try_or_400!(post_input!(request, {
        game_id: i64,
        user_id: String,
        amount: i64,
    }));

    let mut game = game_mut.lock().expect("Error while locking mutex!");
    let res = match game.save_transaction(data.game_id, &data.user_id.as_str(), data.amount) {
        Ok(_) => rouille::Response::text("Ok").with_status_code(200),
        Err(err) => rouille::Response::text(err.to_string()).with_status_code(500),
    };

    return res;
}

fn handle_get_transaction(game_mut: &Mutex<Game>, _request: &Request) -> rouille::Response {
    let mut game = game_mut.lock().expect("Error while locking mutex!");

    let transactions = game
        .get_all_transactions()
        .expect("Couldn't get transactions");

    rouille::Response::json(&transactions)
        .with_additional_header("Access-Control-Allow-Origin", "*")
}

fn handle_get_profit(game_mut: &Mutex<Game>, _request: &Request) -> rouille::Response {
    let mut game = game_mut.lock().expect("Error while locking mutex!");

    let profit = game.get_profit().expect("Couldn't get profits");

    rouille::Response::json(&profit).with_additional_header("Access-Control-Allow-Origin", "*")
}

fn handle_get_profit_by_day(game_mut: &Mutex<Game>, _request: &Request) -> rouille::Response {
    let mut game = game_mut.lock().expect("Error while locking mutex!");

    let profit = game.get_profit_by_day().expect("Couldn't get profits");

    rouille::Response::json(&profit).with_additional_header("Access-Control-Allow-Origin", "*")
}
