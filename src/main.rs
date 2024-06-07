use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod articles;
use articles::services;

struct AppState {
    articles_list: Mutex<Vec<ArticleEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ArticleEntry {
    id: i32,
    title: String,
    data: String,
}

#[get("/")]
async fn index() -> String {
    "This is a health check for the server".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        articles_list: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
