use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub bitcoin_address: String,
    #[serde(default)]
    pub gender: String,
    #[serde(default)]
    pub ip_address: String,
    #[serde(default)]
    pub organization: String,
}
