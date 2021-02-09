use actix_web::{
    get, web, guard, App, HttpServer, HttpResponse, Responder,
};
use std::sync::Mutex;

async fn index() -> impl Responder {
    "Hello world!"
}


// 所有相同scope内路由和资源共享的状态
struct AppState {
    app_name: String,
}

#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}", app_name)
}


struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}


// configure
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/config")
            .route(web::get().to(|| HttpResponse::Ok().body("config")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    println!("Listening on: {}", address);
    let counter_data = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            // .data(AppState {
            //     app_name: "Actix-web".to_string(),
            // })
            .configure(config)
            .app_data(counter_data.clone()) // web::Data底层使用Arc
                                            // 为避免创建两个Arc
                                            // 在创建数据前使用app_data注册
            .service(
                web::scope("/app")
                    .configure(scoped_config)
                    .guard(guard::Header("Host", "127.0.0.1:8080"))
                    .route("/index.html", web::get().to(index)),
            )
            // .service(state)
            .route("/counter", web::get().to(counter))
    })
    .bind(address)?
    .run()
    .await
}