use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::env;

#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    let headers: HashMap<String, String> = req.headers()
        .iter()
        .map(|(name, value)| {
            (name.to_string(), value.to_str().unwrap_or("").to_string())
        })
        .collect();
    
    HttpResponse::Ok().json(headers)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Lire le port depuis les variables d'environnement ou utiliser 8080 par défaut
    let port = env::var("PING_LISTEN_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("Server running on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(ping)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}