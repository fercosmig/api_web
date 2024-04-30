use actix_web::*;

mod routes;
use routes::ping::*;
use routes::info::*;
use routes::catalogo::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>
{
    let api = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(ping))
        .route("/info", web::get().to(info))
        .route("/cat", web::get().to(catalogo))
    });
    let porta = 9091;
    let api = api.bind(format!("127.0.0.1:{}" ,porta)).expect("não conectou");
    println!("Conectado com sucesso\nhttp::\\localhost:{}", porta);

    api.run().await
}
