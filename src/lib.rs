mod git_repository;

use std::collections::HashMap;

use axum::{Json, Router, http::HeaderMap, routing::post};
use git2::Repository;

#[derive(Debug, Clone)]
pub struct AppState {}

pub fn router() -> Router<AppState> {
    Router::new().route("/webhook", post(handle_webhook))
}

#[derive(serde::Deserialize, Debug)]
struct PushWebHook {
    after: String,
    repository: serde_json::Value,
}

#[derive(Debug)]
struct BuildInfo {
    commit_sha: String,
    id: String,
}

async fn handle_webhook(header: HeaderMap, Json(hook): Json<PushWebHook>) {
    println!("Got hook: {hook:?}");
    let info = BuildInfo {
        commit_sha: hook.after,
        id: header
            .get("X-GitHub-Delivery")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };
    println!("{info:?}");
}

async fn run_build() {
    // TODO: how do i get url from webhook?
    // Repository::clone(url, into)
    // Check out repo
    // Build
    // Test
    // Collect result and publish
}
