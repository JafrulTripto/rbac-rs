use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Id,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: Id,
    pub name: String,
}
