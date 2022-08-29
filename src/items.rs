use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub trait Item {

    fn serialize(&self) -> Value;

    fn name(&self) -> &str;

    fn price(&self) -> i32;

    fn set_price(&mut self, price: i32);

    fn make_sound(&self) -> &str {
        return "Detta föremål gör inget ljud!";
    }

}

#[derive(Serialize, Deserialize)]
pub struct Car {
    pub price: i32,
    pub motor: String,
}

impl Item for Car {

    fn name(&self) -> &str {
        return "Bil";
    }

    fn price(&self) -> i32 {
        return self.price;
    }

    fn set_price(&mut self, price: i32) {
        self.price = price;
    }

    fn make_sound(&self) -> &str {
        return "Vroom vroom!";
    }

    fn serialize(&self) -> Value {
        return json!({
            "type": "Car",
            "motor": self.motor
        });
    }
}

#[derive(Serialize, Deserialize)]
pub struct Fox {
    pub price: i32,
}

impl Item for Fox {
    fn name(&self) -> &str {
        return "Räv";
    }

    fn set_price(&mut self, price: i32) {
        self.price = price;
    }

    fn price(&self) -> i32 {
        return self.price;
    }

    fn serialize(&self) -> Value {
        return json!({
            "type": "Fox"
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dog {
    pub price: i32,
    pub length: u32,
}

impl Dog {
    pub fn new(price: i32, length: u32) -> Self {
        Self {
            price,
            length,
        }
    }
}

impl Item for Dog {
    fn name(&self) -> &str {
        return "Hund";
    }

    fn price(&self) -> i32 {
        return self.price;
    }

    fn set_price(&mut self, price: i32) {
        self.price = price;
    }

    fn make_sound(&self) -> &str {
        return "Voff voff!";
    }

    fn serialize(&self) -> Value {
        return json!({
            "type": json!("Dog")
        })
    }
}
