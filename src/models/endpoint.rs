use super::datarecord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub description: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeEndpoint {
    pub msg: String,
    pub link: HashMap<String, String>,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.link, self.description)
    }
}

pub trait ResponseParser {
    fn parse_data(&self) -> Vec<datarecord::DataRecord>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataEndpoint {
    pub data: String,
    #[serde(default)]
    pub link: HashMap<String, String>,
    #[serde(default)]
    pub mime_type: String,
}
