pub mod common;

pub mod detect_objects;

pub use detect_objects::{DetectObjects, DetectObjectsResponseBodyOkJson};
//
pub const SUBSCRIPTION_KEY_HEADER_KEY_NAME: &str = "Ocp-Apim-Subscription-Key";
pub const MODEL_VERSION_QUERY_NAME: &str = "model-version";
