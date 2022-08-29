pub mod items;
pub mod store; 

use crate::items::Car;
use crate::items::Item;
use crate::items::Fox;
use crate::items::Dog;
use crate::store::MarketPlaceStore;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Stdin;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

pub struct MarketPlace {
    items: Vec<Box<dyn Item>>,
}

impl MarketPlace {
    fn list_items(&self) {
        println!("Listar föremål ({} st): ", self.items.len());
        for (index, item) in self.items.iter().enumerate() {
            println!(
                "[{}] > {} för {} kr: {}",
                index,
                item.name(),
                item.price(),
                item.make_sound()
            );
        }
    }

    fn create_item(&mut self, input: &Stdin, item_type: &str){
        match item_type {
            "Car" => {
                let price = match parse_prompt::<i32>(&input, "Hur mycket kostar bilen?") {
                    Some(price) => price,
                    None => return,
                };

                let motor = prompt(&input, "Vad har bilen för motor?");

                self.items.push(Box::new(Car{
                    price,
                    motor
                }));
            }

            "Fox" => {
                let price = match parse_prompt::<i32>(&input, "Hur mycket kostar räven?") {
                    Some(price) => price,
                    None => return,
                }; 

                self.items.push(Box::new(Fox {
                   price 
                }));
            }

            "Dog" => {
                let price = match parse_prompt::<i32>(&input, "Hur mycket kostar hunden?") {
                    Some(price) => price,
                    None => return,
                };

                let length = match parse_prompt::<i32>(&input, "Hur lång är hunden?") {
                    Some(length) => length,
                    None => return,
                };

                self.items.push(Box::new(Dog::new(price, length as u32)));
            }

            &_ => {
                println!("Okänd föremålstyp!");
                return;
            }
        }

         println!("Lade till ett nytt föremål!");
    }

    fn change_price(&mut self, index: usize, price: i32) -> bool {
        if let Some(item) = self.items.get_mut(index){
            item.set_price(price);
            return true;
        }

        return false;
    }

    fn remove_item(&mut self, index: usize) -> Option<Box<dyn Item>> {
        if index >= self.items.len(){
            return None;
        }

        return Some(self.items.remove(index));
    }

}

fn parse<T: FromStr>(str: &str) -> Option<T> {
    return match str.trim_end().parse::<T>() {
        Ok(int) => Some(int),
        Err(_e) => {
            println!("Felaktigt format!");
            return None
        },
    };
}

fn parse_prompt<T: FromStr>(input: &Stdin, question: &str) -> Option<T> {
    return parse::<T>(&prompt(&input, &question));
}

fn prompt(input: &Stdin, question: &str) -> String {
    print!("> {} ", question);
    std::io::stdout().flush().unwrap();
    
    let mut line = String::new();
    input.read_line(&mut line).unwrap();

    return line.trim_end().to_string();
}

fn read_marketplace() -> MarketPlace {
    if Path::new("marketplace.json").exists() {
        let mut buff = String::new();
        let mut file = File::open("marketplace.json").expect("Kunde inte läsa marketplace filen");
        file.read_to_string(&mut buff).unwrap();

        let deserialized: MarketPlaceStore = serde_json::from_str(&buff).expect("Kunde inte deserialisera marketplace");
        return deserialized.generate_market_place();
    } else {

        File::create("marketplace.json").expect("Marketplace kunde inte skapas");
        OpenOptions::new()
            .write(true)
            .open("marketplace.json")
            .expect("Marketplace filen kunde inte öppnas med rätt behörigheter.")
            .write_all("[]".as_bytes())
            .expect("Kunde inte skriva data till marketplace filen");

        return MarketPlace {
            items: vec![]
        }
    }
}

fn main() {
    let mut marketplace = read_marketplace();

    println!("Välkommen till Bosses loppis!");
    println!("Skriv \"h\" för hjälp.\n\n");

    let input = std::io::stdin();
    loop {
        let line = prompt(&input, "");
        let args = line.split(" ").collect::<Vec<&str>>();

        match args[0] {
            "l" => {
                marketplace.list_items();
            }

            "h" => {
                println!("l - Lista alla föremål");
                println!("a <typ> - Lägg till ett föremål");
                println!("d <id> - Ta bort ett föremål");
                println!("cp <id> <pris> - Ändra priset på ett föremål");
                println!("s - Spara loppisen till fil");
                println!(" ");
            }

            "a" => {
                if args.len() != 2 {
                    continue;
                }

                marketplace.create_item(&input, args[1]);
            }

            "cp" => {
                if args.len() != 3 {
                    continue;
                }

                let (index, price) = (
                    match parse::<usize>(&args[1]){
                        Some(int) => int, None => continue},

                    match parse::<i32>(&args[2])
                        {Some(int) => int, None => continue}
                );

                if !marketplace.change_price(index, price){
                    println!("Okänt föremål!");
                    continue;
                }

                println!("Ändrade priset på föremålet till {} kr", price);
            }
            "d" => {
                if args.len() != 2 {
                    continue;
                }

                let index = 
                    match parse::<usize>(&args[1]){
                        Some(int) => int, None => continue};

                let item = match marketplace.remove_item(index){
                    Some(item) => item, 
                    None => {
                        println!("Okänt föremål!");
                        continue;
                    }
                };

                println!("Tog bort en {} från loppisen!", item.name());
            }
            "s" => {
                let store = MarketPlaceStore::from_market_place(&marketplace);
                let mut file_save = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("marketplace.json")
                        .unwrap();

                file_save.write_all(serde_json::to_string(&store).unwrap().as_bytes())
                         .expect("Kunde inte spara marketplace filen");

                println!("Sparade loppisen till fil.");
            }
            _ => {
                println!("Okänt kommando, använd \"h\" för hjälp.");
            }
        }
    }
}