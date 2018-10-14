extern crate cards;
extern crate game;

fn main() {
    let game = game::Game::new();
    let max_col = game.layout.iter().map(|col| {
        col.cards_in_play.len()
    }).max().unwrap();
    for y in 0..max_col {
        let result = game.layout.iter().fold("".to_string(), |line, col| {
            let entry = match col.cards_in_play.len() {
                len if len > y &&  y >= len - col.visible_count => format!("{}", col.cards_in_play[y]),
                len if len > y  => format!("(----)"),
                _ => format!{"          "}
            };
            format!("{} {:<10}", line, entry)
        });
        println!("{}", result);
    };
}
