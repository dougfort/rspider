extern crate hex;
extern crate cards;
extern crate client;

use std::io::{stdin, stdout};
use std::io::Write; 

fn main() {
    let mut client: Option<client::Client> = None;
    loop {
        /*
        display_game(&game);
        println!("");
        display_moves(&game);
        */
        println!("");
        print!("> ");
        stdout().flush().unwrap();


        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let split_input: Vec<&str> = input.trim().split(' ').collect();
        match split_input[0].trim() {
            "" => continue,
            "quit" => break,
            "new" => {
                client = if split_input.len() > 1 {
                    match client::Client::from_hex(split_input[1]) {
                        Ok(c) => Some(c),
                        Err(e) => {
                            println!("unable to create game from seed: {}, {}", split_input[1], e);
                            None
                        }
                    }
                } else {
                    Some(client::Client::new())
                }
            }
            _ => {
                println!("invalid input");
                continue;
            },
        }

        println!("");
        match client {
            None => println!("no game in progress"),
            Some(c) => {
                println!("game = {}", c.seed());
            },
        }
    }
}

/*
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

*/