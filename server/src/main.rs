use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main(
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest_service("/", ServeDir::new(PathBuf::from("dist")));

    Ok(router.into())
}
