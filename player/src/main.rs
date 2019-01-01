extern crate hex;
extern crate cards;
extern crate client;

use std::error::Error;
use std::io::{stdin, stdout};
use std::io::Write; 

fn main() -> Result<(), Box<Error>> {
    let mut client = match std::env::args().skip(1).next() {
        Some(seed) => client::Client::from_hex(&seed)?,
        None => client::Client::new()?
    };
    loop {
        display_local_game(&client);
        display_possible_moves(&client)?;

        let stdin_line = get_stdin_line(">")?;
        let command: Vec<&str> = stdin_line.trim().split_whitespace().collect();   
        if command.is_empty() {
            continue;
        }
        match command[0].trim() {
            "" => continue,
            "quit" => break,
            "deal" => {
                if client.cards_dealt() < client.total_cards() {
                    client.deal()?;
                };
            },
            "checkpoints" => {
                for cp in client.checkpoints() {
                    println!("{:?}", cp);
                }
            }
            "undo" => {
                if client.checkpoints().len() < 2 {
                    println!("nothing to undo");
                    continue;
                };
                client.undo()?;
            }
            _ => {
                println!("invalid input");
                continue;
            },
        }
    }

    Ok(())
}

fn get_stdin_line(prompt: &str) -> std::io::Result<String> {
    println!("");
    print!("{} ", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input)
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

fn display_possible_moves(client: &client::Client) -> Result<(), Box<Error>>{
    println!("");
    println!("possible moves");
    println!("");
    for m in client.possible_moves()? {
        println!("{:?}", m);
    };

    Ok(())
}