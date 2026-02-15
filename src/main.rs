use ci_rust::{AppState, router};

#[tokio::main]
async fn main() {
    let app_state = AppState {};
    let app = router().with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
