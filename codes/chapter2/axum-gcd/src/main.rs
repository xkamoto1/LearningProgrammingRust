use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

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
