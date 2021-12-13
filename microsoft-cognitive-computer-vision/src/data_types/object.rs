use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub rectangle: ObjectRectangle,
    pub object: String,
    pub confidence: f64,
    pub parent: ObjectParent,
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
