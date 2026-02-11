use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post};

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

#[post("/order")]
async fn create_order() -> impl Responder {
    HttpResponse::Ok().body("Order created")
}

#[delete("/order")]
async fn delete_order() -> impl Responder {
    HttpResponse::Ok().body("Order deleted")
}

#[get("/depth")]
async fn get_depth() -> impl Responder {
    HttpResponse::Ok().body("Market depth")
}
