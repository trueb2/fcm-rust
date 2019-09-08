pub mod response;

pub use crate::client::response::*;

use futures::stream::TryStreamExt;
use http::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, RETRY_AFTER};
use hyper::{
    client::{Client as HttpClient, HttpConnector},
    Body, Request, StatusCode,
};
use hyper_tls::{self, HttpsConnector};
use crate::message::Message;
use serde_json;

/// An async client for sending the notification payload.
pub struct Client {
    http_client: HttpClient<HttpsConnector<HttpConnector>>,
}

impl Client {
    /// Get a new instance of Client.
    pub fn new() -> Result<Client, hyper_tls::Error> {
        let mut http_client = HttpClient::builder();
        http_client.keep_alive(true);

        Ok(Client {
            http_client: http_client.build(HttpsConnector::new().unwrap()),
        })
    }

    /// Try sending a `Message` to FCM.
    pub async fn send(&self, message: Message<'_>) -> Result<FcmResponse, FcmError> {
        let payload = serde_json::to_vec(&message.body).unwrap();

        let mut builder = Request::builder();

        builder.method("POST");
        builder.header(CONTENT_TYPE, "application/json");
        builder.header(
            CONTENT_LENGTH,
            format!("{}", payload.len() as u64).as_bytes(),
        );
        builder.header(AUTHORIZATION, format!("key={}", message.api_key).as_bytes());
        builder.uri("https://fcm.googleapis.com/fcm/send");

        let request = builder.body(Body::from(payload)).unwrap();
        let response = self.http_client.request(request).await?;
        let response_status = response.status();

        let retry_after = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|ra| ra.to_str().ok())
            .and_then(|ra| RetryAfter::from_str(ra));

        let body = response.into_body().try_concat().await?;

        match response_status {
            StatusCode::OK => {
                let fcm_response: FcmResponse = serde_json::from_slice(&body).unwrap();

                match fcm_response.error {
                    Some(ErrorReason::Unavailable) => {
                        Err(response::FcmError::ServerError(retry_after))
                    }
                    Some(ErrorReason::InternalServerError) => {
                        Err(response::FcmError::ServerError(retry_after))
                    }
                    _ => Ok(fcm_response),
                }
            }
            StatusCode::UNAUTHORIZED => Err(response::FcmError::Unauthorized),
            StatusCode::BAD_REQUEST => Err(response::FcmError::InvalidMessage(
                "Bad Request".to_string(),
            )),
            status if status.is_server_error() => {
                Err(response::FcmError::ServerError(retry_after))
            }
            _ => Err(response::FcmError::InvalidMessage(
                "Unknown Error".to_string(),
            )),
        }
    }
}
