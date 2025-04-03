use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    destination: String,
    source: Option<String>,
    options: Option<Vec<String>>,
    #[serde(rename = "type")]
    typ: String
    // uidMappings: Option<Vec<Mapping>>,
    // gidMappings: Option<Vec<Mapping>>
}