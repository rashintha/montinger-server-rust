use rocket::{
    fairing::{Fairing, Info, Kind},
    http::uri::Origin,
    Data, Request,
};

use crate::util::util_jwt::decode_access_jwt;

pub struct JWTAuthFairing;

#[rocket::async_trait]
impl Fairing for JWTAuthFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT Authentication Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        if request.uri().path() == "/auth/login" || request.uri().path() == "/auth/refresh" {
            return; // Skip auth for login route
        }

        // let state = request.guard::<State<AppState>>().await.expect("AppState not managed");
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            let claims_result = decode_access_jwt(auth_header.trim_start_matches("Bearer "));

            match claims_result {
                Ok(claims) => {
                    if claims.exp > chrono::Utc::now().timestamp() {
                        // Token is valid, you can add the claims to the request's local cache
                        request.local_cache(|| claims);
                        return;
                    }
                }
                Err(_) => {}
            }
        }

        let uri = Origin::parse("/error/unauthorized").unwrap();

        // If the token is invalid or not present, return an Unauthorized error
        request.set_uri(uri);
    }
}
