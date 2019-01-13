extern crate failure;
use failure::Error;

pub fn help() {
    println!("quit: exit game");
    println!("deal: deal one card face up on each pile");
    println!("move <n>:Doug Fort Consulting, Inc. execute one of the numbered moves");
    println!("checkpoints: list the known checkpoints");
    println!("undo: undo the previous operation");
    println!("auto [<n>]: play in auto mode, for at most 'n' moves");
}

pub fn local_game(client: &client::Client) {
    println!("");
    println!("game: {}; cards dealt: {}; cards remaining: {}", client.seed(), client.cards_dealt(), client.total_cards() - client.cards_dealt());
    println!("");    
    println!("");
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

pub fn possible_moves(client: &client::Client) -> Result<(), Error>{
    println!("");
    println!("possible moves");
    println!("");
    for (i, m) in client.possible_moves()?.iter().enumerate() {
        println!("{}: {:?}", i+1, m);
    };

    Ok(())
}
