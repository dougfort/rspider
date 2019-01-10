use std::error::Error;
use std::{thread, time};

use display;

pub fn play(client: &mut client::Client) -> Result<(), Box<Error>> {
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
            let action = moves[1];            
            println!(">>> {:?}", action);
            let current_move = (action.orig_col, action.count, action.dest_col);
            if current_move == prev_move {
                println!("*** duplicate move");
            };
            client.move_cards(action)?;
            prev_move = current_move;
        }

        display::local_game(&client);
        display::possible_moves(&client)?;

        let wait_time = time::Duration::from_secs(2);
        thread::sleep(wait_time);
    }

    Ok(())
}