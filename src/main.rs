mod db;
mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use db::connect_db;
use routes::{create_task, get_tasks};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = connect_db().await;

    println!("Server is started running on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
