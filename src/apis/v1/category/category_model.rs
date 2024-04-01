use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow,Deserialize, Serialize)]

pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(sqlx::FromRow,Deserialize, Serialize)]

pub struct NewCategory {
    pub id: i32,
    pub name: String,
}