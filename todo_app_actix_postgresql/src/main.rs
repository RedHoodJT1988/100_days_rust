use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use dotenvy::dotenv;
 
#[derive(Serialize, Deserialize, FromRow)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}
 
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}
 
#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}
 
async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(pool.get_ref())
        .await;
 
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to fetch todos"),
    }
}
 
async fn get_one(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await;
 
    match result {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("❌ Todo not found"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Query failed"),
    }
}
 
async fn create_todo(data: web::Json<CreateTodo>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query("INSERT INTO todos (title) VALUES ($1)")
        .bind(&data.title)
        .execute(pool.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Created().body("✅ Todo created"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Failed to create todo"),
    }
}
 
async fn update_todo(
    id: web::Path<i32>,
    data: web::Json<UpdateTodo>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let current = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await;
 
    if let Ok(Some(todo)) = current {
        let new_title = data.title.clone().unwrap_or(todo.title);
        let new_completed = data.completed.unwrap_or(todo.completed);
 
        let result = sqlx::query("UPDATE todos SET title = $1, completed = $2 WHERE id = $3")
            .bind(new_title)
            .bind(new_completed)
            .bind(*id)
            .execute(pool.get_ref())
            .await;
 
        match result {
            Ok(_) => HttpResponse::Ok().body("🔄 Todo updated"),
            Err(_) => HttpResponse::InternalServerError().body("❌ Update failed"),
        }
    } else {
        HttpResponse::NotFound().body("❌ Todo not found")
    }
}
 
async fn delete_todo(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await;
 
    match result {
        Ok(_) => HttpResponse::Ok().body("🗑️ Todo deleted"),
        Err(_) => HttpResponse::InternalServerError().body("❌ Delete failed"),
    }
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL not set");
    let db = PgPool::connect(&db_url).await.expect("❌ Failed to connect to DB");
 
    println!("✅ Todo App running at http://localhost:8080");
 
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/todos", web::get().to(get_all))
            .route("/todos/{id}", web::get().to(get_one))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}