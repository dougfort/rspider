extern crate failure;
extern crate hex;
extern crate cards;
extern crate client;

use failure::Error;
use std::io::{stdin, stdout};
use std::io::Write; 

mod auto;
mod display;

fn main() -> Result<(), Error> {
    let mut client = match std::env::args().skip(1).next() {
        Some(seed) => client::Client::from_hex(&seed)?,
        None => client::Client::new()?
    };
    loop {
        display::local_game(&client);
        display::possible_moves(&client)?;

        let stdin_line = get_stdin_line(">")?;
        let command: Vec<&str> = stdin_line.trim().split_whitespace().collect();   
        if command.is_empty() {
            continue;
        }
        match command[0].trim() {
            "" => continue,
            "help" => display::help(),
            "quit" => break,
            "deal" => {
                if client.cards_dealt() < client.total_cards() {
                    client.deal()?;
                };
            },
            "move" => {
                if command.len() < 2 {
                    println!("you must specify the number of a move");
                    continue
                }
                match command[1].parse::<usize>() {
                    Ok(n) => {
                        let moves = client.possible_moves()?;
                        if n-1 >= moves.len() {
                            println!("move number {} out ot bounds", n);
                            continue;
                        }
                        client.move_cards(moves[n-1])?;
                    },
                    Err(e) => {
                        println!("invalid move number {}", e);
                        continue;
                    }
                };

            }
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
            "auto" => {
                auto::play(&mut client)?;
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


