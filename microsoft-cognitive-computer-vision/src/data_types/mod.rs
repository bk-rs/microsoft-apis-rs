pub mod image_input;
pub mod metadata;
pub mod model_version;
pub mod model_version_input;
pub mod object;
pub mod request_id;

pub use image_input::ImageInput;
pub use metadata::Metadata;
pub use model_version::ModelVersion;
pub use model_version_input::ModelVersionInput;
pub use object::Object;
pub use request_id::RequestId;

//
pub type SubscriptionKey = String;
