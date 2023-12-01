use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub async fn handler(Path(path): Path<String>) -> impl IntoResponse {
    let path_numbers = path
        .split_terminator("/")
        .map(|p| p.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut starting_number: i32 = 0;
    for number in path_numbers.iter() {
        starting_number = starting_number ^ number;
    }

    (StatusCode::OK, format!("{}", starting_number.pow(3)))
}
