extern crate failure;

use failure::Error;
use std::{thread, time};

use display;

pub fn play(client: &mut client::Client) -> Result<(), Error> {
    let mut play = 0;
    let mut prev_move = (0, 0, 0);

    'play_loop: loop {
        play += 1;
        println!("play: {}", play);

        let moves = client.possible_moves()?;
        if moves.is_empty() {
            if client.cards_dealt() == client.total_cards() {
                println!("cards exhausted");
                break 'play_loop;
            } else {
                println!(">>> dealing");
                client.deal()?;
            }
        } else {  
            let mut moved = false;          
            'move_loop: for (i, action) in moves.iter().enumerate() {
                println!(">>> {:?}[{}]", action, i);
                let current_move = (action.orig_col, action.count, action.dest_col);
                if current_move == prev_move {
                    println!("*** duplicate move");
                    continue 'move_loop;
                };
                let current_move_reversed = (action.dest_col, action.count, action.orig_col);
                if current_move_reversed == prev_move {
                    println!("*** move cycle");
                    continue 'move_loop;
                };
                client.move_cards(*action)?;
                prev_move = current_move;
                moved = true;
                break 'move_loop;
            }
            if !moved {
                println!(">>> dealing (unable to find valid move)");
                client.deal()?;
            }
        }

        display::local_game(&client);
        display::possible_moves(&client)?;

        let wait_time = time::Duration::from_secs(2);
        thread::sleep(wait_time);
    }

    Ok(())
}