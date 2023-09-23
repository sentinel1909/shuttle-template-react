use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_static_folder::StaticFolder(folder = "dist")] dist_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest_service("/", ServeDir::new(dist_folder));

    Ok(router.into())
}
