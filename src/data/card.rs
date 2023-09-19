#[derive(Clone)]
pub struct CardBuilder {
    name: Option<String>,
    edition: Option<String>,
    vendor: Option<String>,
    rarity: Option<String>,
    condition: Option<String>,
    price: Option<f32>,
    quantity: Option<i32>,
}

impl CardBuilder {
    pub fn new() -> CardBuilder {
        CardBuilder {
            name: None,
            edition: None,
            vendor: None,
            rarity: None,
            condition: None,
            price: None,
            quantity: None,
        }
    }

    pub fn name(mut self, name: &str) -> CardBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn edition(mut self, edition: &str) -> CardBuilder {
        self.edition = Some(edition.to_string());
        self
    }

    pub fn vendor(mut self, vendor: &str) -> CardBuilder {
        self.vendor = Some(vendor.to_string());
        self
    }

    pub fn rarity(mut self, rarity: &str) -> CardBuilder {
        self.rarity = Some(rarity.to_string());
        self
    }

    pub fn condition(mut self, condition: &str) -> CardBuilder {
        self.condition = Some(condition.to_string());
        self
    }

    pub fn price(mut self, price: f32) -> CardBuilder {
        self.price = Some(price);
        self
    }

    pub fn quantity(mut self, quantity: i32) -> CardBuilder {
        self.quantity = Some(quantity);
        self
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name.unwrap_or_else(|| "".to_string()),
            edition: self.edition.unwrap_or_else(|| "".to_string()),
            vendor: self.vendor.unwrap_or_else(|| "".to_string()),
            rarity: self.rarity.unwrap_or_else(|| "".to_string()),
            condition: self.condition.unwrap_or_else(|| "".to_string()),
            price: self.price.unwrap_or(0.0),
            quantity: self.quantity.unwrap_or(0),
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub edition: String,
    pub vendor: String,
    pub rarity: String,
    pub condition: String,
    pub price: f32,
    pub quantity: i32,
}
