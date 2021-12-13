use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub rectangle: ObjectRectangle,
    pub object: String,
    pub confidence: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ObjectParent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectRectangle {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectParent {
    pub object: String,
    pub confidence: f64,
}
