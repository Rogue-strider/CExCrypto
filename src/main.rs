use actix_web::{web, App, HttpResponse, HttpServer, Responder, delete, get, post};
use serde::{Deserialize, Serialize};

use crate::routes::{create_order, delete_order, get_depth};
pub mod routes;
pub mod input;
pub mod output;

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
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}


