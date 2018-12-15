extern crate hex;
extern crate cards;
extern crate game;

use std::io::{stdin};

fn main() {
    let seed = game::seed::from_random();
    let mut game = game::Game::from_seed(seed);

    loop {
        display_game(&game);
        println!("");
        display_moves(&game);

        println!("");
        println!("> ");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        println!("command = {}", command);
    }
}

fn display_game(game: &game::Game) {
    println!("");
    println!("game: {}", hex::encode(game.seed));
    println!("");
    println!(" {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10}", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

    let max_col = game.layout.iter().map(|col| {
        col.cards_in_play.len()
    }).max().unwrap();

    for y in 0..max_col {
        let result = game.layout.iter().fold("".to_string(), |line, col| {
            let entry = match col.cards_in_play.len() {
                len if len > y &&  y >= len - col.visible_count => format!("{}", col.cards_in_play[y]),
                len if len > y  => format!("(----)"),
                _ => format!{"          "}
            };
            format!("{} {:<10}", line, entry)
        });
        println!("{}", result);
    };
}

fn display_moves(game: &game::Game) {
    let moves = game.possible_moves().unwrap();
    for m in moves {
        println!("{:?}", m);
    }
}