use actix_web::{get, web, App, HttpServer, Responder};

#[get("/github")]
async fn greet() -> impl Responder {
    format!("Hello github!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 4567))?
    .run()
    .await
}
