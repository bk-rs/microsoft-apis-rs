use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub width: usize,
    pub height: usize,
    pub format: String,
}
