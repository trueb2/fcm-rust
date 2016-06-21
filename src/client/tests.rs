use {Client, FcmError};
use hyper::status::StatusCode;

#[test]
fn should_parse_error_as_unauthorized() {
    let result = Client::parse_response(StatusCode::Unauthorized, "Unauthorized");

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), FcmError::Unauthorized);
}

#[test]
fn should_parse_error_as_invalid_message() {
    let result = Client::parse_response(StatusCode::BadRequest, "INVALID_REGISTRATION");

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(),
    FcmError::InvalidMessage("INVALID_REGISTRATION".to_string()));
}

#[test]
fn should_parse_error_as_server_error() {
    let result = Client::parse_response(StatusCode::InternalServerError, "Internal Server Error");

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), FcmError::ServerError);
}

#[test]
fn should_parse_successful_response() {
    let response = r#"
        {
            "message_id": 2000000,
            "results": [
                {
                    "message_id": 200000,
                    "registration_id": 200000,
                    "error": "error"
                }
            ]
        }
    "#;
    let result = Client::parse_response(StatusCode::Ok, response);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result.message_id.unwrap(), 2000000);

    let message_results = result.results.unwrap();

    assert_eq!(message_results.len(), 1);
}
