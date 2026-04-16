#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde_json::json;

    use crate::{routes::build_router, state::AppState};

    fn test_server() -> TestServer {
        let state = AppState::new();
        let app = build_router(state);
        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn health_check_returns_ok() {
        let server = test_server();
        let res = server.get("/health").await;
        res.assert_status_ok();
        let body = res.json::<serde_json::Value>();
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn list_items_empty_on_start() {
        let server = test_server();
        let res = server.get("/items").await;
        res.assert_status_ok();
        let body = res.json::<Vec<serde_json::Value>>();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn create_and_get_item() {
        let server = test_server();

        let res = server
            .post("/items")
            .json(&json!({ "name": "Widget", "description": "A handy widget", "price": 9.99 }))
            .await;
        res.assert_status(axum::http::StatusCode::CREATED);
        let item = res.json::<serde_json::Value>();
        let id = item["id"].as_str().unwrap();

        let res = server.get(&format!("/items/{id}")).await;
        res.assert_status_ok();
        let fetched = res.json::<serde_json::Value>();
        assert_eq!(fetched["name"], "Widget");
        assert_eq!(fetched["price"], 9.99);
    }

    #[tokio::test]
    async fn update_item() {
        let server = test_server();

        let res = server
            .post("/items")
            .json(&json!({ "name": "Old Name", "description": "desc", "price": 1.0 }))
            .await;
        let item = res.json::<serde_json::Value>();
        let id = item["id"].as_str().unwrap();

        let res = server
            .put(&format!("/items/{id}"))
            .json(&json!({ "name": "New Name", "price": 2.5 }))
            .await;
        res.assert_status_ok();
        let updated = res.json::<serde_json::Value>();
        assert_eq!(updated["name"], "New Name");
        assert_eq!(updated["price"], 2.5);
        assert_eq!(updated["description"], "desc");
    }

    #[tokio::test]
    async fn delete_item() {
        let server = test_server();

        let res = server
            .post("/items")
            .json(&json!({ "name": "Temp", "description": "temp", "price": 0.0 }))
            .await;
        let item = res.json::<serde_json::Value>();
        let id = item["id"].as_str().unwrap();

        let res = server.delete(&format!("/items/{id}")).await;
        res.assert_status(axum::http::StatusCode::NO_CONTENT);

        let res = server.get(&format!("/items/{id}")).await;
        res.assert_status(axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_nonexistent_item_returns_404() {
        let server = test_server();
        let fake_id = "00000000-0000-0000-0000-000000000000";
        let res = server.get(&format!("/items/{fake_id}")).await;
        res.assert_status(axum::http::StatusCode::NOT_FOUND);
    }
}
