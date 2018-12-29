extern crate hex;
extern crate cards;
extern crate client;

use std::error::Error;
use std::io::{stdin, stdout};
use std::io::Write; 

fn main() {
    loop {
        let stdin_line = match get_stdin_line(">") {
            Err(e) => {
                println!("ERROR: get_stdin_line{:?}", e);
                continue;
            },
            Ok(c) => c,
        };
        let command: Vec<&str> = stdin_line.trim().split_whitespace().collect();   
        if command.is_empty() {
            continue;
        }
        match command[0].trim() {
            "" => continue,
            "quit" => break,
            "new" => {
                match client_loop(&command[1..]) {
                    Err(e) => {
                        println!("ERROR: client loop{:?}", e);
                    },
                    Ok(()) => {},
                }
            },
            _ => {
                println!("invalid input");
                continue;
            },
        }
    }
}

fn get_stdin_line(prompt: &str) -> std::io::Result<String> {
    println!("");
    println!("{} ", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input)
}
 
fn client_loop(client_params: &[&str]) -> Result<(), Box<Error>> {
    let client = if client_params.len() > 1 {
        client::Client::from_hex(client_params[1])?
    } else {
        client::Client::new()?
    };

    loop {
        display_local_game(&client);

        let stdin_line = match get_stdin_line(">>") {
            Err(e) => {
                println!("ERROR: get_stdin_line{:?}", e);
                continue;
            },
            Ok(c) => c,
        };
        let command: Vec<&str> = stdin_line.trim().split_whitespace().collect();   
        if command.is_empty() {
            continue;
        }
        if command.is_empty() {
            continue;
        }
        match command[0].trim() {
            "" => continue,
            "quit" => break,
            _ => {
                println!("invalid input");
                continue;
            },
        }
   }

    Ok(())
}

fn display_local_game(client: &client::Client) {
    println!("");
    println!("game: {}; cards dealt: {}; cards remaining: {}", client.seed(), client.cards_dealt(), client.total_cards() - client.cards_dealt());
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
