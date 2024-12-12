use axum::{
    extract::Form,
    handler::{get, post},
    http::header,
    response::{Html, IntoResponse},
    Router,
};
use std::net::SocketAddr;
use chrono::Utc;
use rinja::Template;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct SignUpForm {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file
    dotenv::dotenv().ok();

    // Build our application with a route
    let app = Router::new()
        .route("/", get(show_landing_page))
        .route("/signup", post(handle_signup));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn show_landing_page() -> impl IntoResponse {
    let template = Template::new("src/templates/index.html").unwrap();
    let current_year = Utc::now().year().to_string();
    let content = template.render_with_context(&current_year).unwrap();

    (
        [(header::CACHE_CONTROL, "public, max-age=900")],
        Html(content),
    )
}

async fn handle_signup(Form(form): Form<SignUpForm>) -> impl IntoResponse {
    // Get the Google Sheet URL from the environment variable
    let google_sheet_url = env::var("GOOGLE_SHEET_URL").expect("GOOGLE_SHEET_URL must be set");

    // Prepare the data to be sent to the Google Sheet
    let data = serde_json::json!({
        "name": form.name,
        "email": form.email,
    });

    // Send the data to the Google Sheet
    let client = reqwest::Client::new();
    let res = client
        .post(&google_sheet_url)
        .json(&data)
        .send()
        .await;

    match res {
        Ok(_) => Html("<p>Thank you for signing up!</p>".to_string()),
        Err(_) => Html("<p>Failed to sign up. Please try again later.</p>".to_string()),
    }
}