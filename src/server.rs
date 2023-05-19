use std::sync::{Mutex, Arc};

use actix_web::{post, web::{Data, self}, HttpResponse, get};
use serde::Deserialize;

use crate::file_handler::{file_store::FileStore, file_info::{FileInfo, DirInfo}};

#[derive(Deserialize)]
pub struct Path {
    path: String
}

#[post("/edit_file")]
pub async fn edit_file(state: Data<Arc<Mutex<FileStore>>>, data: web::Json<FileInfo>) -> HttpResponse {
    if let Err(err) = state.lock().unwrap().edit_file(data.0) {
        return HttpResponse::BadRequest().body(serde_json::to_string_pretty(&err).unwrap());
    }
    return HttpResponse::Ok().finish();
}

#[post("/read_file")]
pub async fn read_file(state: Data<Arc<Mutex<FileStore>>>, data: web::Json<Path>) -> HttpResponse {
    match state.lock().unwrap().read_file(data.0.path) {
        Err(err) =>  return HttpResponse::BadRequest().body(serde_json::to_string_pretty(&err).unwrap()),
        Ok(body) => return HttpResponse::Ok().body(serde_json::to_string_pretty(&body).unwrap())
    }
}

#[post("/write_dir")]
pub async fn write_dir(data: web::Json<DirInfo>) -> HttpResponse {
    if let Err(err) = FileStore::write_dir(data.0) {
        return HttpResponse::BadRequest().body(serde_json::to_string_pretty(&err).unwrap())
    }

    return HttpResponse::Ok().finish();
}

#[post("/read_dir")]
pub async fn read_dir(state: Data<Arc<Mutex<FileStore>>>, data: web::Json<Path>) -> HttpResponse {
    match state.lock().unwrap().read_dir(data.0.path) {
        Ok(body) => return HttpResponse::Ok().body(serde_json::to_string_pretty(&body).unwrap()),
        Err(err) => HttpResponse::BadRequest().body(serde_json::to_string(&err).unwrap()),
    }
}

#[get("/")]
pub async fn index() -> HttpResponse {
    return HttpResponse::Ok().body("Upload file to get started")
} 