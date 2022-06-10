use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
// FromRequest
pub struct BasicAuthStruct {
    pub username: String,
    pub password: String,
}

// Basic username:password
impl BasicAuthStruct {
    fn from_header(header: &str) -> Option<BasicAuthStruct> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Basic" {
            return None;
        }
        // base64
        Self::from_base64(split_vec[1])
    }

    fn from_base64(base64_string: &str) -> Option<BasicAuthStruct> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split_vec = decoded_str.split(":").collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        let (username, password) = (split_vec[0].to_string(), split_vec[1].to_string());
        Some(BasicAuthStruct { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuthStruct {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_header(header_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}