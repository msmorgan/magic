extern crate scryfall;

use self::scryfall::client;
use self::scryfall::object::*;

use std::fs::File;
use std::io::Write;

fn main() {
    let list = client::get("https://api.scryfall.com/bulk-data")
        .unwrap();

    let mut bulk_data_link = None;

    if let Object::List(list) = list {
        for item in list.data() {
            if let Object::BulkData(item) = item {
                if item.type_() == "oracle_cards" {
                    bulk_data_link = Some(item.permalink_uri().to_string());
                    break;
                }
            }
        }
    }

    let mut card_names = Vec::new();

    if let Some(bulk_data_link) = bulk_data_link {
        let cards = client::get_bulk(&bulk_data_link)
            .unwrap();

        for card in cards.iter() {
            if let Object::Card(card) = card {
                card_names.push(card.name().to_string());
            }
        }
    }


    let filename = "card_names.txt";
    let mut file = File::create(filename).unwrap();
    for name in card_names.iter() {
        writeln!(file, "{}", name).unwrap();
    }
    println!("wrote file: {}", filename);

    let card_names_catalog = client::get("https://api.scryfall.com/catalog/card-names")
        .unwrap()
        .into_catalog()
        .unwrap();


    let filename = "card_names_catalog.txt";
    let mut file = File::create(filename).unwrap();
    for name in card_names_catalog.data() {
        writeln!(file, "{}", name).unwrap();
    }
    println!("wrote file: {}", filename);

//    let result = client::get(
//        "https://api.scryfall.com/cards/search?q=pow%3d9%20tou%3d3")
//        .unwrap();
//
//    if let Some(list) = result.as_list() {
//        for card in list.data().filter_map(Object::as_card) {
//            println!("{}", card.name());
//        }
//    }
//
//    println!("{:#?}", result);
}
