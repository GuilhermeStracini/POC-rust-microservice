use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    models::{CreateItemRequest, Item, UpdateItemRequest},
    state::SharedState,
};

pub async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "poc-rust-microservice" }))
}

pub async fn list_items(State(state): State<SharedState>) -> Json<Vec<Item>> {
    let items = state.items.read().await;
    let list: Vec<Item> = items.values().cloned().collect();
    Json(list)
}

pub async fn get_item(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, StatusCode> {
    let items = state.items.read().await;
    items
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_item(
    State(state): State<SharedState>,
    Json(req): Json<CreateItemRequest>,
) -> (StatusCode, Json<Item>) {
    let item = Item::new(req);
    let mut items = state.items.write().await;
    items.insert(item.id, item.clone());
    (StatusCode::CREATED, Json(item))
}

pub async fn update_item(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateItemRequest>,
) -> Result<Json<Item>, StatusCode> {
    let mut items = state.items.write().await;
    match items.get_mut(&id) {
        Some(item) => {
            item.apply_update(req);
            Ok(Json(item.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_item(State(state): State<SharedState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut items = state.items.write().await;
    if items.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
