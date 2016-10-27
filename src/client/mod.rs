pub mod response;

pub use client::response::*;
use std::io::Read;
use message::Message;
use hyper::client::Client as HttpClient;
use hyper::header::{Authorization, ContentType, Connection};
use hyper::status::StatusCode;
use rustc_serialize::json::{self, ToJson};
use retry_after::RetryAfter;

pub struct Client {
    http_client: HttpClient,
}

impl Client {
    /// Get a new instance of Client.
    pub fn new() -> Client {
        Client {
            http_client: HttpClient::new(),
        }
    }

    /// Send a message using your FCM API Key.
    /// # Examples:
    /// ```no_run
    /// use fcm::{MessageBuilder, Client};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let message = MessageBuilder::new("<registration id>").data(map).finalize();
    /// let client = Client::new();
    /// let result = client.send(message, "<FCM API Key>");
    /// ```
    pub fn send(&self, message: Message, api_key: &str) -> Result<FcmResponse, FcmError> {
        let payload = message.to_json().to_string();
        let auth_header = format!("key={}", api_key);

        let response = self.http_client.
            post("https://fcm.googleapis.com/fcm/send").
            header(Connection::keep_alive()).
            header(ContentType::json()).
            body(&payload).
            header(Authorization(auth_header)).
            header(ContentType::json()).
            send();

        match response {
            Ok(mut response) => {
                let mut body = String::new();
                let retry_after = response.headers.get::<RetryAfter>().map(|ra| *ra);
                response.read_to_string(&mut body).unwrap();

                match response.status {
                    StatusCode::Ok => {
                        let fcm_response: FcmResponse = json::decode(&body).unwrap();

                        match fcm_response.error.as_ref().map(String::as_ref) {
                            Some("Unavailable") =>
                                return Err(response::FcmError::ServerError(retry_after)),
                            Some("InternalServerError") =>
                                return Err(response::FcmError::ServerError(retry_after)),
                            _ => ()
                        }

                        Result::Ok(fcm_response)
                    },
                    StatusCode::Unauthorized =>
                        Err(response::FcmError::Unauthorized),
                    StatusCode::BadRequest =>
                        Err(response::FcmError::InvalidMessage(body.to_string())),
                    status if status.is_server_error() =>
                        Err(response::FcmError::ServerError(retry_after)),
                    _ =>
                        Err(response::FcmError::InvalidMessage("Unknown Error".to_string())),
                }
            },
            Err(_) => {
                Err(response::FcmError::ServerError(None))
            }
        }
    }
}
