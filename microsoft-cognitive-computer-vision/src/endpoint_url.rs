#[derive(Debug, Clone)]
pub enum EndpointUrl {
    ResourceName(String),
    WestUS,
}

impl EndpointUrl {
    pub fn url(&self) -> String {
        match self {
            EndpointUrl::ResourceName(name) => {
                format!("https://{}.cognitiveservices.azure.com/", name)
            }
            EndpointUrl::WestUS => "https://westus.api.cognitive.microsoft.com/".to_owned(),
        }
    }
}
