use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow,Deserialize, Serialize)]

pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category_name: String,  // Foreign key reference to the Category table
}

#[derive(sqlx::FromRow,Deserialize, Serialize)]
pub struct NewProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category_name: String,  // Foreign key reference to the Category table
}