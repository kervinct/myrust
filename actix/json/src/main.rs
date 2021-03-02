use actix_web::{error, post, web, App, Error, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;  // 256K

#[post("/")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<Person>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(index_manual)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}