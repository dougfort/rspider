extern crate cards;

fn main() {
    let deck = cards::deck();
    println!("card = {:?}", deck);
}
