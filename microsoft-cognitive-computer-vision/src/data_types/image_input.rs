use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ImageInput {
    inner: ImageInputInner,
}

#[derive(Debug, Clone)]
enum ImageInputInner {
    Url(String),
    Raw(Vec<u8>),
}

impl ImageInput {
    pub fn with_url(url: impl AsRef<str>) -> Result<Self, String> {
        let url = url.as_ref();

        // TODO, check url schema

        Ok(Self {
            inner: ImageInputInner::Url(url.to_owned()),
        })
    }

    pub fn with_raw(bytes: impl AsRef<[u8]>) -> Result<Self, String> {
        let bytes = bytes.as_ref();

        // TODO, check
        /*
        Supported image formats: JPEG, PNG, GIF, BMP.
        Image file size must be less than 4MB.
        Image dimensions should be greater than 50 x 50.
        */

        Ok(Self {
            inner: ImageInputInner::Raw(bytes.to_owned()),
        })
    }

    pub fn to_request_body_and_content_type(&self) -> Result<(Vec<u8>, &'static str), String> {
        match &self.inner {
            ImageInputInner::Url(url) => {
                let body = ImageInputRequestJsonBody {
                    url: url.to_owned(),
                };
                let body =
                    serde_json::to_vec(&body).map_err(|err| format!("ser failed, err: {}", err))?;
                Ok((body, "application/json"))
            }
            ImageInputInner::Raw(bytes) => Ok((bytes.to_owned(), "application/octet-stream")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageInputRequestJsonBody {
    pub url: String,
}
