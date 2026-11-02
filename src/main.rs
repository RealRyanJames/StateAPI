use actix_web::{get, HttpServer, App, Responder, HttpResponse, web};

#[get("/")]
async fn main_route() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello Ryan From Rust & Cargo"))
}

async fn route() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello Ryan From Rust & Cargo"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            main_route
        ).route("/", web::get().to(route))
    }).bind(("127.0.0.1", 3000))?.run().await
}