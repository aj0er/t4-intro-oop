use crate::MarketPlace;
use crate::items::*;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct MarketPlaceStore {
    items: Vec<Value>
}

fn construct_item_type<T: DeserializeOwned + 'static + Item>(val: &Value) -> Option<Box<dyn Item>> {
    let item: T = serde_json::from_value::<T>(val.clone()).unwrap();
    return Some(Box::new(item));
}

fn construct_item(val: &Value) -> Option<Box<dyn Item>> {
    return match val["type"].as_str().unwrap() {
        "Car" => construct_item_type::<Car>(val),
        "Fox" => construct_item_type::<Fox>(val), 
        "Dog" => construct_item_type::<Dog>(val),
        _ => None
    }
}

fn construct_value(item: &Box<dyn Item>) -> Value {
    let mut val = item.serialize();
    let serialized = val.as_object_mut().unwrap();
    serialized.insert("price".to_string(), json!(item.price()));

    return json!(serialized);
}

impl MarketPlaceStore {
    pub fn from_market_place(market_place: &MarketPlace) -> Self {
        return MarketPlaceStore {
            items: market_place.items.iter().map(|x| construct_value(x)).collect::<Vec<Value>>()
        }
    }

    pub fn generate_market_place(&self) -> MarketPlace {
        let mut market_place = MarketPlace {
            items: vec![]
        };

        for val in self.items.iter() {
            if let Some(item) = construct_item(val) {
                market_place.items.push(item);
            }
        }

        return market_place;
    }
}

