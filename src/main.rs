use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use validator::Validate;

use crate::models::BuyPizzaRequest; // Importing the required modules
mod models; // Importing the models module

#[get("/pizzas")]

async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizza are available")
}

#[post("/buypizza")]
async fn buy_pizza(body:Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            return HttpResponse::Ok().body(format!("Pizza bought125: {}", pizza_name));
            // println!("Validated");
        }
        Err(_) =>  HttpResponse::Ok().body("Pizza name required"),
            // println!("Error: {}", e);
        
        
    }
    // HttpResponse::Ok().body("Pizza bought")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizza updated")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
