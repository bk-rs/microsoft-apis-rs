/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p microsoft-cognitive-computer-vision-demo --bin microsoft_cognitive_computer_vision_detect_objects -- 'YOUR_SUBSCRIPTION_KEY' 'YOUR_ENDPOINT_RESOURCE_NAME'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use microsoft_cognitive_computer_vision::{
    data_types::ImageInput, operations::DetectObjects, EndpointUrl,
};

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let subscription_key = env::args().nth(1).unwrap();
    let endpoint_resource_name = env::args().nth(2).unwrap();
    let image_bytes = include_bytes!("../../tests/image_files/2.jpeg");

    let client = IsahcClient::new()?;

    let detect_objects = DetectObjects::new(
        EndpointUrl::ResourceName(endpoint_resource_name),
        &subscription_key,
        ImageInput::with_raw(image_bytes)?,
        None,
    );

    let ret = client.respond_endpoint(&detect_objects).await?;
    println!("{:?}", ret);

    Ok(())
}
