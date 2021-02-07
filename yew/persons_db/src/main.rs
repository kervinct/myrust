use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

mod db_access;
use db_access::DbPrivilege;


struct AppState {
    db: db_access::DbConnection,
}

fn check_credentials(
    auth: BasicAuth, 
    state: &web::Data<Mutex<AppState>>,
    required_privilege: DbPrivilege,
) -> Result<Vec<DbPrivilege>, String> {
    let db_conn = &state.lock().unwrap().db;
    if let Some(user) = db_conn.get_user_by_username(auth.user_id()) {
        if auth.password().is_some() && &user.password == auth.password().unwrap() {
            if user.privileges.contains(&required_privilege) {
                Ok(user.privileges.clone())
            } else {
                Err(format!(
                    "Insufficient privileges for user \"{}\".",
                    user.username
                ))
            }
        } else {
            Err(format!("Invalid password for user \"{}\".", user.username))
        }
    } else {
        Err(format!("User \"{}\" not found.", auth.user_id()))
    }
}

#[derive(Serialize)]
struct AuthenticationResult {
    user: db_access::User,
}

async fn authenticate(auth: BasicAuth, state: web::Data<Mutex<AppState>>) -> impl Responder {
    println!("In authenticate");
    let db_conn = &state.lock().unwrap().db;
    if let Some(user) = db_conn.get_user_by_username(auth.user_id()) {
        if auth.password().is_some() && &user.password == auth.password().unwrap() {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json!(AuthenticationResult { user: user.clone() }).to_string())
        } else {
            HttpResponse::Forbidden()
                .content_type("application/json")
                .body(
                    json!(
                        &format!(
                            "Invalid password for user \"{}\".", 
                            user.username,
                        )
                    )
                    .to_string()
                )
        }
    } else {
        HttpResponse::Forbidden()
            .content_type("application/json")
            .body(
                json!(&format!("User \"{}\".", auth.user_id()))
                .to_string()
            )
    }
}

async fn get_person_by_id(
    auth: BasicAuth,
    state: web::Data<Mutex<AppState>>,
    web::Path((id,)): web::Path<(u32,)>,
) -> impl Responder {
    println!("In get_person_by_id");
    match check_credentials(auth, &state, DbPrivilege::CanRead) {
        Err(msg) => HttpResponse::Forbidden()
            .content_type("application/json")
            .body(json!(&msg).to_string()),
        Ok(_) => {
            let db_conn = &state.lock().unwrap().db;
            if let Some(person) = db_conn.get_person_by_id(id) {
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json!(person).to_string())
            } else {
                HttpResponse::NotFound().finish()
            }
        }
    }
}

#[derive(Deserialize)]
struct Filter {
    partial_name: Option<String>,
}

async fn get_persons(
    query: web::Query<Filter>,
    auth: BasicAuth,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("In get_persons");
    match check_credentials(auth, &state, DbPrivilege::CanRead) {
        Ok(_) => {
            let partial_name = &query.partial_name.clone().unwrap_or_else(|| "".to_string());
            let db_conn = &state.lock().unwrap().db;
            HttpResponse::Ok()
                .content_type("application/json")
                .body(
                    json!(db_conn
                        .get_persons_by_partial_name(partial_name)
                        .collect::<Vec<_>>()
                    ).to_string(),
                )
        }
        Err(msg) => HttpResponse::Forbidden()
            .content_type("application/json")
            .body(json!(&msg).to_string()),
    }
}

#[derive(Deserialize)]
struct ToDelete {
    id_list: Option<String>,
}

async fn delete_persons(
    query: web::Query<ToDelete>,
    auth: BasicAuth,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("In delete_persons: {:?}", query.id_list);
    match check_credentials(auth, &state, DbPrivilege::CanWrite) {
        Ok(_) => {
            let db_conn = &mut state.lock().unwrap().db;
            let mut deleted_count = 0;
            query.id_list
                .clone()
                .unwrap_or_else(|| "".to_string())
                .split_terminator(',')
                .for_each(|id| {
                    deleted_count += if db_conn.delete_by_id(id.parse::<u32>().unwrap()) {
                        1
                    } else {
                        0
                    };
                });
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json!(deleted_count).to_string())
        }
        Err(msg) => HttpResponse::Forbidden()
            .content_type("application/json")
            .body(json!(&msg).to_string()),
    }
}

#[derive(Deserialize)]
struct PersonData {
    id: Option<String>,
    name: Option<String>,
}

async fn insert_person(
    state: web::Data<Mutex<AppState>>,
    query: web::Query<PersonData>,
    auth: BasicAuth,
) -> impl Responder {
    println!("In insert_person");
    match check_credentials(auth, &state, DbPrivilege::CanWrite) {
        Err(msg) => HttpResponse::Forbidden()
            .content_type("application/json")
            .body(json!(&msg).to_string()),
        Ok(_) => {
            let db_conn = &mut state.lock().unwrap().db;
            let name = query.name.clone().unwrap();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(
                    json!(
                        db_conn
                        .insert_person(db_access::Person{ id: 0, name }))
                    .to_string()
                )
        }
    }
}

async fn update_person(
    auth: BasicAuth,
    query: web::Query<PersonData>,
    state: web::Data<Mutex<AppState>>,
) -> impl Responder {
    println!("In insert_person");
    match check_credentials(auth, &state, DbPrivilege::CanWrite) {
        Err(msg) => HttpResponse::Forbidden()
            .content_type("application/json")
            .body(json!(&msg).to_string()),
        Ok(_) => {
            let db_conn = &mut state.lock().unwrap().db;
            let name = query.name.clone().unwrap_or_else(|| "".to_string()).clone();
            let id = query.id.clone().unwrap();
            let id = str::parse::<u32>(&id).unwrap();
            println!("In update_person: id={:?}, name={:?}", id, name);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(
                    json!(db_conn.update_person(db_access::Person { id, name }))
                    .to_string()
                )
        }
    }
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

async fn invalid_method(req: HttpRequest) -> impl Responder {
    println!("Invalid method {} for URI: \"{}\"", req.method(), req.uri());
    HttpResponse::NotFound()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8000";
    println!("Listening at address: {}", server_address);
    let db_conn = web::Data::new(Mutex::new(AppState {
        db: db_access::DbConnection::new(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(db_conn.clone())
            .data(Config::default().realm("PersonsApp"))
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION, 
                        header::ACCEPT, 
                    ])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .service(
                web::resource("/authenticate")
                    .route(web::get().to(authenticate))
                    .default_service(web::route().to(invalid_method)),
            )
            .service(
                web::resource("/person/id/{id}")
                    .route(web::get().to(get_person_by_id))
                    .default_service(web::route().to(invalid_method)),
            )
            .service(
                web::resource("/one_person")
                    .route(web::post().to(insert_person))
                    .route(web::put().to(update_person))
                    .default_service(web::route().to(invalid_method)),
            )
            .service(
                web::resource("/persons")
                    .route(web::get().to(get_persons))
                    .route(web::delete().to(delete_persons))
                    .default_service(web::route().to(invalid_method)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
    .await
}
