use axum::{routing::get, Router};

mod routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route(
            "/1/*numbers",
            get(crate::routes::one::sled_id_system::handler),
        )
        .route("/", get(crate::routes::hello_world::handler))
        .route("/-1/error", get(crate::routes::fake_error::handler));

    Ok(router.into())
}
