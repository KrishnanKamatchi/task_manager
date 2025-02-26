use crate::models::Task;
use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_tasks(pool: web::Data<SqlitePool>) -> impl Responder {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(tasks)
}

use serde_json::json; // Import serde_json for custom JSON responses

pub async fn create_task(pool: web::Data<SqlitePool>, task: web::Json<Task>) -> impl Responder {
    let new_task = Task {
        id: Uuid::new_v4().to_string(), // Convert UUID to String
        title: task.title.clone(),
        description: task.description.clone(),
        completed: false,
    };

    sqlx::query("INSERT INTO tasks (id, title, description, completed) VALUES (?, ?, ?, ?)")
        .bind(&new_task.id)
        .bind(&new_task.title)
        .bind(&new_task.description)
        .bind(new_task.completed)
        .execute(pool.get_ref())
        .await
        .unwrap();

    let response = json!({
        "success": true,
        "message": "Task created successfully",
        "task_id": new_task.id
    });

    HttpResponse::Created().json(response)
}
