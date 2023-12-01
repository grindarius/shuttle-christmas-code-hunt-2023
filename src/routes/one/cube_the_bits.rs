use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CubeTheBitsPath {
    first_number: u32,
    second_number: u32,
}

pub async fn handler(path: Path<CubeTheBitsPath>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("{}", (path.first_number ^ path.second_number).pow(3)),
    )
}
