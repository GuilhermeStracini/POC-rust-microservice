use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}

impl Item {
    pub fn new(req: CreateItemRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            price: req.price,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn apply_update(&mut self, req: UpdateItemRequest) {
        if let Some(name) = req.name {
            self.name = name;
        }
        if let Some(description) = req.description {
            self.description = description;
        }
        if let Some(price) = req.price {
            self.price = price;
        }
        self.updated_at = Utc::now();
    }
}
