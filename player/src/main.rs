extern crate hex;
extern crate cards;
extern crate client;

use std::io::{stdin, stdout};
use std::io::Write; 

fn main() {
    let mut client: Option<client::Client> = None;
    loop {
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
                let client_result = if split_input.len() > 1 {
                    client::Client::from_hex(split_input[1])
                } else {
                    client::Client::new()
                };
                client = match client_result {
                    Ok(c) => Some(c),
                    Err(e) => {
                        println!("unable to create client: {}", e);
                        None
                    }
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
            Some(c) => display_local_game(&c),
        }
    }
}

fn display_local_game(client: &client::Client) {
    println!("");
    println!("game: {}; cards dealt: {}", client.seed(), client.cards_dealt());
    println!("");    println!("");
    println!(" {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10} {:^10}", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

    let max_col = client.local.iter().map(|col| {
        col.len()
    }).max().unwrap();

    for y in 0..max_col {
        let result = client.local.iter().fold("".to_string(), |line, col| {
            let entry = if y < col.len() {
                match col[y] {
                    Some(card) => format!("{}", card),
                    None => format!("(----)")
                }
            } else {
                format!{"          "}
            };
            format!("{} {:<10}", line, entry)
        });
        println!("{}", result);
    };
}
/*
fn display_moves(game: &game::Game) {
    let moves = game.possible_moves().unwrap();
    for m in moves {
        println!("{:?}", m);
    }
}
*/
