//! https://westus.dev.cognitive.microsoft.com/docs/services/computer-vision-v3-2/operations/5e0cdeda77a84fcd9a6d4e1b

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Method,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    data_types::{
        ImageInput, Metadata, ModelVersion, ModelVersionInput, Object, RequestId, SubscriptionKey,
    },
    operations::{
        common::{endpoint_parse_response, EndpointError, EndpointRet},
        MODEL_VERSION_QUERY_NAME, SUBSCRIPTION_KEY_HEADER_KEY_NAME,
    },
    EndpointUrl,
};

//
//
//
#[derive(Debug, Clone)]
pub struct DetectObjects {
    endpoint_url: EndpointUrl,
    subscription_key: SubscriptionKey,
    image: ImageInput,
    model_version: Option<ModelVersionInput>,
}
impl DetectObjects {
    pub fn new(
        endpoint_url: EndpointUrl,
        subscription_key: impl AsRef<str>,
        image: ImageInput,
        model_version: impl Into<Option<ModelVersionInput>>,
    ) -> Self {
        Self {
            endpoint_url,
            subscription_key: subscription_key.as_ref().to_owned(),
            image,
            model_version: model_version.into(),
        }
    }
}

impl Endpoint for DetectObjects {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<DetectObjectsResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("{}vision/v3.2/detect", self.endpoint_url.url());
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        if let Some(model_version) = &self.model_version {
            url.query_pairs_mut()
                .append_pair(MODEL_VERSION_QUERY_NAME, model_version.to_string().as_str());
        }

        let (body, content_type) = self
            .image
            .to_request_body_and_content_type()
            .map_err(EndpointError::ImageInputToRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url.as_str())
            .header(SUBSCRIPTION_KEY_HEADER_KEY_NAME, &self.subscription_key)
            .header(CONTENT_TYPE, content_type)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(body)
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        endpoint_parse_response(response)
    }
}

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DetectObjectsResponseBodyOkJson {
    pub objects: Vec<Object>,
    pub request_id: RequestId,
    pub metadata: Metadata,
    pub model_version: ModelVersion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body_ok_json() {
        match serde_json::from_str::<DetectObjectsResponseBodyOkJson>(include_str!(
            "../../tests/response_body_files/detect_objects_ok.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.objects.len(), 1);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
