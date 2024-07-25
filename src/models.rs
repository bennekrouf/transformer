use serde::{Deserialize, Serialize};

// Define the data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub path: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub endpoints: Vec<Endpoint>,
    pub fields: Vec<Field>,
}
