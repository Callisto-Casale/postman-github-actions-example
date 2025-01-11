use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Struct voor een eenvoudige databank
#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

// Shared state
struct AppState {
    items: Mutex<Vec<Item>>,
}

// GET: /status
#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "API is running" }))
}

// POST: /items
#[post("/items")]
async fn create_item(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Created().finish()
}

// GET: /items
#[get("/items")]
async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

// GET: /items/{id}
#[get("/items/{id}")]
async fn get_item_by_id(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner(); // Pak de waarde van id uit als een kopieerbare u32
    let items = data.items.lock().unwrap();
    if let Some(item) = items.iter().find(|item| item.id == id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({ "error": "Item not found" }))
    }
}

#[put("/items/{id}")]
async fn update_item(
    id: web::Path<u32>,
    updated_item: web::Json<Item>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = id.into_inner(); // Pak de waarde van id uit als een kopieerbare u32
    let mut items = data.items.lock().unwrap();
    if let Some(item) = items.iter_mut().find(|item| item.id == id) {
        *item = updated_item.into_inner();
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({ "error": "Item not found" }))
    }
}

#[delete("/items/{id}")]
async fn delete_item(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner(); // Pak de waarde van id uit als een kopieerbare u32
    let mut items = data.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|item| item.id == id) {
        items.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(serde_json::json!({ "error": "Item not found" }))
    }
}

// Main functie
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        items: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(status)
            .service(create_item)
            .service(get_items)
            .service(get_item_by_id)
            .service(update_item)
            .service(delete_item)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
