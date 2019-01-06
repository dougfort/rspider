use std::error::Error;


pub fn play(client: &mut client::Client) -> Result<(), Box<Error>> {
    let mut play = 0;

    loop {
        play += 1;
        println!("play: {}", play);

        let moves = client.possible_moves()?;
        if moves.is_empty() {

        }
    }

    Ok(())
}