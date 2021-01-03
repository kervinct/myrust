use actix_web::{web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

async fn delete_file(Path((filename,)): Path<(String,)>) -> impl Responder {
    print!("Deleting file \"{}\" ... ", filename);
    flush_stdout();

    match std::fs::remove_file(&filename) {
        Ok(_) => {
            println!("Deleted file \"{}\"", filename);
            HttpResponse::Ok()
        }
        Err(error) => {
            println!("Failed to delete file \"{}\": {}", filename, error);
            HttpResponse::NotFound()
        }
    }
}

async fn download_file(Path((filename,)): Path<(String,)>) -> impl Responder {
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();

    fn read_file_content(filename: &str) -> std::io::Result<String> {
        use std::io::Read;
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    match read_file_content(&filename) {
        Ok(contents) => {
            println!("Downloaded file \"{}\"", filename);
            HttpResponse::Ok().content_type("text/plain").body(contents)
        }
        Err(error) => {
            println!("Failed to read file \"{}\": {}", filename, error);
            HttpResponse::NotFound().finish()
        }
    }
}

async fn upload_specified_file(
    mut body: web::Payload,
    Path((filename,)): Path<(String,)>,
) -> impl Responder {
    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let f = File::create(&filename);

    if f.is_err() {
        println!("Failed to create file \"{}\"", filename);
        return HttpResponse::NotFound().into();
    }

    if f.unwrap().write_all(&bytes).is_err() {
        println!("Failed to write file \"{}\"", filename);
        return HttpResponse::NotFound().into();
    }

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok().finish()
}

async fn upload_new_file(
    mut body: web::Payload,
    Path((filename,)): Path<(String,)>,
) -> impl Responder {
    print!("Uploading file \"{}*.txt\" ... ", filename);
    flush_stdout();

    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let mut rng = rand::thread_rng();
    let mut attempts = 0;
    let mut file;
    let mut fname;
    const MAX_ATTEMPTS: u32 = 100;

    loop {
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            println!(
                "Failed to create new file with prefix \"{}\", \
                after {} attempts.",
                filename, MAX_ATTEMPTS
            );
            return HttpResponse::NotFound().into();
        }
        fname = format!("{}{:03}.txt", filename, rng.gen_range(0..1000));

        file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&fname);
        
        if file.is_ok() {
            break;
        }
    }

    if file.unwrap().write_all(&bytes).is_err() {
        println!("Failed to write file \"{}\"", filename);
        return HttpResponse::NotFound().into();
    }

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(filename)
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {} ...", server_address);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_specified_file))
                    .route(web::post().to(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_address)?
    .run()
    .await
}