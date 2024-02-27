// use std::fmt::format;

use actix_web::web::Data;
use actix_web::{web::Path, get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use surrealdb::sql::Uuid;
use validator::Validate;
use crate::db::Database;
use uuid;
use crate::models::{BuyPizzaRequest, UpdatePizzaUrl,Pizza}; // Importing the required modules
mod models;

mod db ;
mod error;

#[get("/pizzas")]
async fn get_pizzas(db:Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        None => HttpResponse::Ok().body("No pizzas found")
    }
    // HttpResponse::Ok().body("Pizzas")
}

#[post("/buypizza")]
async fn buy_pizza(body:Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer =uuid::Uuid::encode_buffer();
            let new_uuid=uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = db.add_pizza(Pizza::new(String::from(new_uuid), pizza_name)).await;
            match new_pizza{
                Some(created)=>{
                    HttpResponse::Ok().body(format!("Pizza bought: {:?}", created))

                
                },
                None=>
                    HttpResponse::Ok().body("Error buying pizza")
            }
        }
            //  HttpResponse::Ok().body(format!("Pizza bought125: {}", pizza_name))
            // println!("Validated");
        
        Err(_) =>  HttpResponse::Ok().body("Pizza name required"),
            // println!("Error: {}", e);
        
        
    }
    // HttpResponse::Ok().body("Pizza bought")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url:Path<UpdatePizzaUrl>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid.clone();
    HttpResponse::Ok().body(format!("Update pizza with uuid: {}", uuid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("Database connection failed");
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
