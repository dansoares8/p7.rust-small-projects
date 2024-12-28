use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]


puc struct User{
    pub id: u64,
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String
}