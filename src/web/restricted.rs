use maud::{html, Markup, DOCTYPE};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get, 
    Router,
    response::Html,
};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/restricted", get(self::get::restricted))
}

mod get {
    use super::*;

    pub async fn restricted(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => Html(restricted_page(&user.username).into_string()).into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

fn restricted_page(username: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { title { "Restricted" } }
            body {
                p {
                    "Logged in as " (username) ", who has the "
                    code { "\"restricted.read\"" }
                    " permission."
                }
            }
        }
    }
}
