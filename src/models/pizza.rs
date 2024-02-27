use serde :: {Deserialize, Serialize};
use validator :: Validate;
#[derive(Validate, Serialize, Deserialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 3,message = "Name should be at least 3 characters"))]
    pub pizza_name: String,
    // #[validate(range(min = 100, max = 5000))]
    // pub price: f32,
    // #[validate(range(min = 1, max = 5))]
    // pub quantity: i32,
}