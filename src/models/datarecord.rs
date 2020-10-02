use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataRecord {
    id: usize,
    first_name: String,
    last_name: String,
    full_name: String,
    email: String,
    employee_id: String,
    bitcoin_address: String,
    gender: String,
    ip_address: String,
    organization: String,
}
