use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("👋 Hello, Rust Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello) // Register the route handler
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}