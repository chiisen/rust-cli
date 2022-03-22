use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    println!("程式啟動!");

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
