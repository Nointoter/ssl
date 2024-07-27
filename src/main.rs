use actix_web::{web, App, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, HTTPS!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Настройка SSL
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("private.key", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}