use chrono::prelude::*;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub name: String,
    pub email: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

impl FilteredUser {
    // pub fn new_user(user: &User) -> Self {
    //     let created_at_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(user.created_at.unwrap(), Utc);
    //     let updated_at_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(user.updated_at.unwrap(), Utc);
    //     Self {
    //         // id: user.id,
    //         email: user.email.to_owned(),
    //         name: user.name.to_owned(),
    //         // photo: user.photo.to_owned(),
    //         // role: user.role.to_owned(),
    //         verified: user.verified,
    //         createdAt: created_at_utc,
    //         updatedAt: updated_at_utc,
    //     }
    // }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}
