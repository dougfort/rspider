use std::error::Error;


pub fn play(client: &mut client::Client) -> Result<(), Box<Error>> {
    let mut play = 0;

    'play_loop: loop {
        play += 1;
        println!("play: {}", play);

        let moves = client.possible_moves()?;
        if moves.is_empty() {
            if client.cards_dealt() == client.total_cards() {
                println!("cards exhausted");
                break 'play_loop;
            } else {
                client.deal()?;
            }
        } else {
            client.move_cards(moves[1])?;
        }
    }

    Ok(())
}