use actix_web::{get, App, HttpResponse, HttpServer, Responder};
#[macro_use]
extern crate dotenv_codegen;

const PORT: &str = dotenv!("PORT");

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("API running on http://localhost:{}", &PORT);
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("127.0.0.1", PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}
