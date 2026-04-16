use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::Item;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub items: RwLock<HashMap<Uuid, Item>>,
}

impl AppState {
    pub fn new() -> SharedState {
        Arc::new(Self {
            items: RwLock::new(HashMap::new()),
        })
    }
}
