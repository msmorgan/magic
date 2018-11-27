extern crate scryfall;

use self::scryfall::client;

#[test]
fn get_thrummingbird() {
    let obj = client::get("https://api.scryfall.com/cards/named?exact=Thrummingbird")
        .unwrap();

    eprintln!("{:?}", obj);
}