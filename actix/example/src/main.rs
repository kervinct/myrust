use actix_web::{get, post, web, App, HttpServer, HttpRequest, HttpResponse, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(
    web::Path((id, name)): web::Path<(u32, String)>,
) -> impl Responder {
    format!("Hello {}! id: {}", name, id)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}

#[post("/echo")]
async fn echo(req: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(echo)
            .route("/{name}", web::get().to(greet))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
