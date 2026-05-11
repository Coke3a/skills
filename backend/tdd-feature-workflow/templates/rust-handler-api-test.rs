// Template destination:
// tests/api/<feature>_api_test.rs
//
// Use this only when HTTP contract, request/response DTO mapping,
// route wiring, auth extraction, or API error mapping is introduced or changed.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[path = "../common/mod.rs"]
mod common;

use common::build_test_app;

#[tokio::test]
async fn post_invalid_request_returns_400() {
    let app = build_test_app().await; // project-specific placeholder
    let request = Request::builder()
        .method("POST")
        .uri("/example-entities")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "name": "" }).to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn post_valid_request_returns_201() {
    let app = build_test_app().await; // project-specific placeholder
    let request = Request::builder()
        .method("POST")
        .uri("/example-entities")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "name": "Valid name" }).to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn get_missing_entity_returns_404() {
    let app = build_test_app().await; // project-specific placeholder
    let request = Request::builder()
        .method("GET")
        .uri("/example-entities/missing-id")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn response_has_expected_dto_shape() {
    let app = build_test_app().await; // project-specific placeholder
    let request = Request::builder()
        .method("POST")
        .uri("/example-entities")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "name": "Valid name" }).to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("id").is_some());
    assert_eq!(json["name"], "Valid name");
}
