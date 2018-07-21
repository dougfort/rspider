extern crate cards;
extern crate game;

fn main() {
    let game = game::Game::new();
    for card in game.reserve {
        println!("card = {}", card);
    }
}
