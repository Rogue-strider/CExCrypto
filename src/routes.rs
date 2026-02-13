use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web::{self, Json}};

use crate::{CreateOrderInput, CreateOrderResponse, DeleteOrder};



#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id: String::from("ads")
    })
} 

#[delete("/order")]
pub async fn delete_order(order: web::Json<DeleteOrder>) -> impl Responder {
    println!("Delete order: {:?}", order);

    HttpResponse::Ok().json("Order deleted")
}


#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    HttpResponse::Ok().body("Market depth")
}