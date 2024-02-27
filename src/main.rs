// use std::fmt::format;

use actix_web::web::Data;
use actix_web::{web::Path, get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use surrealdb::sql::Uuid;
use validator::Validate;
// use crate::db::Database;
use crate::error::PizzaError;
use crate::db::{pizza_data_trait::PizzaData,Database};
use uuid;
use crate::models::{BuyPizzaRequest, UpdatePizzaUrl,Pizza}; // Importing the required modules
mod models;

mod db ;
mod error;

#[get("/pizzas")]
async fn get_pizzas(db:Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    // let pizzas = db.get_all_pizzas().await;
    let pizzas = Database::get_all_pizzas(&db).await;
    match pizzas {
        // Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        // None => HttpResponse::Ok().body("No pizzas found")
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
    // HttpResponse::Ok().body("Pizzas")
}

#[post("/buypizza")]
async fn buy_pizza(body:Json<BuyPizzaRequest>, db: Data<Database>) -> Result<Json<Pizza>,PizzaError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer =uuid::Uuid::encode_buffer();
            let new_uuid=uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza =
            //  db.add_pizza
            Database::add_pizza
            (&db,Pizza::new(String::from(new_uuid), pizza_name)).await;
            match new_pizza{
                Some(created)=>{
                    Ok(Json(created))
                    // HttpResponse::Ok().body(format!("Pizza bought: {:?}", created))

                
                },
                None=>Err(PizzaError::PizzaCreationFailure),
                    // HttpResponse::Ok().body("Error buying pizza")
            }
        }
            //  HttpResponse::Ok().body(format!("Pizza bought125: {}", pizza_name))
            // println!("Validated");
        
        Err(_) => Err(PizzaError::PizzaCreationFailure),
        //  HttpResponse::Ok().body("Pizza name required"),
            // println!("Error: {}", e);
        
        
    }
    // HttpResponse::Ok().body("Pizza bought")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url:Path<UpdatePizzaUrl>,db:Data<Database>) -> Result<Json<Pizza>,PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let updated_pizza =
    //  db.update_pizza(uuid).await;
    Database::update_pizza(&db,uuid).await;
    // HttpResponse::Ok().body(format!("Update pizza with uuid: {}", uuid))
    match updated_pizza {
        Some(updated_pizza)=>Ok(Json(updated_pizza)),
        None=>Err(PizzaError::NoSuchPizzaFound)
    }
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
