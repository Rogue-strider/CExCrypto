use actix_web::{web, App, HttpResponse, HttpServer, Responder, delete, get, post};
use serde::{Deserialize, Serialize};


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}

#[post("/order")]
async fn create_order(order: web::Json<CreateOrder>) -> impl Responder {
    println!("Received order: {:?}", order);

    HttpResponse::Ok().json("Order created")
}

#[delete("/order")]
async fn delete_order(order: web::Json<DeleteOrder>) -> impl Responder {
    println!("Delete order: {:?}", order);

    HttpResponse::Ok().json("Order deleted")
}


#[get("/depth")]
async fn get_depth() -> impl Responder {
    HttpResponse::Ok().body("Market depth")
}
//1.11