use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    effective: Option<Vec<String>>,
    bounding: Option<Vec<String>>,
    inheritable: Option<Vec<String>>,
    permitted: Option<Vec<String>>,
    ambient: Option<Vec<String>>,
}
