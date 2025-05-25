use axum::{
    Router,
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/gcd", post(post_gcd));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="n" />
        <input type="text" name="m" />
        <button type="submit">Compute GCD</button>
    </form>
    "#,
    )
}

#[derive(Deserialize)]
struct Numbers {
    n: u64,
    m: u64,
}

async fn post_gcd(Form(numbers): Form<Numbers>) -> impl IntoResponse {
    if numbers.n == 0 || numbers.m == 0 {
        return (StatusCode::BAD_REQUEST, Html("Computing the GCD with zero is boring.".to_string()));
    }

    let response = format!(
        "The greatest common divisor of {} and {} is {}.",
        numbers.n,
        numbers.m,
        gcd(numbers.n, numbers.m)
    );
    (StatusCode::OK, Html(response))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0, "GCD with zero is undefined");
    while m != 0 {
        let t = m;
        m = n % m;
        n = t;
    }
    n
}
