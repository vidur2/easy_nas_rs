use std::sync::{Mutex, Arc};

use actix_web::{HttpServer, App};
use easy_nas_rs::{file_handler::file_store::FileStore, server::{edit_file, read_file, index, read_dir, write_dir}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_store = Arc::new(Mutex::new(FileStore::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(Arc::clone(&file_store))
        .service(edit_file)
        .service(read_file)
        .service(read_dir)
        .service(write_dir)
        .service(index)
    }).bind(("0.0.0.0", 5001))?
    .run()
    .await
}