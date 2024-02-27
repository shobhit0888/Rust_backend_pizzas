use crate::models::{Pizza};
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::{Error};

#[async_trait]
pub trait PizzaData {
    async fn get_all_pizzas(db:&Data<Database>) -> Option<Vec<Pizza>>;
    async fn add_pizza(db:&Data<Database>, new_pizza: Pizza) -> Option<Pizza>;
    async fn update_pizza(db:&Data<Database>, uuid: String) -> Option<Pizza>;
}
#[async_trait]
impl PizzaData for Database {
    async fn get_all_pizzas(db:&Data<Database>) -> Option<Vec<Pizza>> {
        let result = db.client.select("pizza").await;
        match result {
            Ok(all_pizzas) =>
                Some(all_pizzas),
            Err(_) => None
        }
    }
    async fn add_pizza(db:&Data<Database>, new_pizza: Pizza) -> Option<Pizza> {
        let created_pizza = db.client.create(("pizza", new_pizza.uuid.clone())).content(new_pizza).await;
        match created_pizza {
            Ok(created) => created,
            Err(_) => None
        }
    }
    async fn update_pizza(db:&Data<Database>, uuid: String) -> Option<Pizza> {
        let find_pizza: Result<Option<Pizza>, Error> = db.client.select(("pizza", &uuid)).await;
        match find_pizza {
            Ok(found_pizza) => {
                match found_pizza {
                    Some(pizza) => {
                        let updated_pizza: Result<Option<Pizza>, Error> = db.client.update(("pizza", &uuid)).merge(Pizza {
                            uuid,
                            pizza_name: String::from("sold")
                        })
                            .await;
                        match updated_pizza {
                            Ok(updated) => updated,
                            Err(_) => None
                        }
                    }
                    None => None
                }
            }
            Err(_) => None
        }
    }
}