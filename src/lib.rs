use tower_service::Service;
use axum::{routing::get, Router};
use worker::*;
use axum::response::Redirect;

// HIDDEN DEPENDENCY: we utilize cf worker's static file server
// I believe this is far from lock-in as it is trivial to add static file serving in another deployment fashion if needed

fn router() -> Router {
    Router::new()
        .route("/", get(Redirect::permanent("/index.html")))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}
