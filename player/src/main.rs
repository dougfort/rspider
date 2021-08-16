use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod auto;
mod display;

fn main() -> Result<()> {
    let mut client = match std::env::args().nth(1) {
        Some(seed) => client::Client::from_hex(&seed)?,
        None => client::Client::new()?,
    };

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        display::local_game(&client);
        display::possible_moves(&client)?;

        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let command: Vec<&str> = line.trim().split_whitespace().collect();
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
                    }
                    "move" => {
                        if command.len() < 2 {
                            println!("you must specify the number of a move");
                            continue;
                        }
                        match command[1].parse::<usize>() {
                            Ok(n) => {
                                let moves = client.possible_moves()?;
                                if n > moves.len() {
                                    println!("move number {} out of bounds", n);
                                    continue;
                                }
                                client.move_cards(moves[n - 1].mv)?;
                            }
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
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
