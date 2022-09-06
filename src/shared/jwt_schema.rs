use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtSchema {
    // pub id: String,
    pub username: String,
}
