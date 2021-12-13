use core::fmt;

use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

//
//
//
#[derive(Debug, Clone)]
pub enum EndpointRet<T>
where
    T: fmt::Debug + Clone,
{
    Ok(T),
    Other((StatusCode, Result<EndpointResponseBodyErrJson, Body>)),
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum EndpointError {
    #[error("MakeRequestUrlFailed {0}")]
    MakeRequestUrlFailed(UrlParseError),
    #[error("ImageInputToRequestBodyFailed {0}")]
    ImageInputToRequestBodyFailed(String),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
}

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndpointResponseBodyErrJson {
    pub error: EndpointResponseBodyErrJsonError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndpointResponseBodyErrJsonError {
    pub code: String,
    pub message: String,
    pub innererror: Option<EndpointResponseBodyErrJsonErrorInnererror>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndpointResponseBodyErrJsonErrorInnererror {
    pub code: String,
    pub message: String,
}

//
//
//
pub fn endpoint_parse_response<T>(response: Response<Body>) -> Result<EndpointRet<T>, EndpointError>
where
    T: fmt::Debug + Clone + DeserializeOwned,
{
    let status = response.status();
    match status {
        StatusCode::OK => {
            let ok_json = serde_json::from_slice::<T>(response.body())
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?;

            Ok(EndpointRet::Ok(ok_json))
        }
        StatusCode::METHOD_NOT_ALLOWED => {
            match serde_json::from_slice::<EndpointResponseBodyErrJsonError>(response.body()) {
                Ok(err_json) => Ok(EndpointRet::Other((
                    status,
                    Ok(EndpointResponseBodyErrJson { error: err_json }),
                ))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            }
        }
        status => match serde_json::from_slice::<EndpointResponseBodyErrJson>(response.body()) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((
                status,
                Err(response.body().to_owned()),
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body_err_json() {
        match serde_json::from_str::<EndpointResponseBodyErrJson>(include_str!(
            "../../tests/response_body_files/detect_objects_err__401.json"
        )) {
            Ok(err_json) => {
                assert_eq!(err_json.error.code, "401");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
