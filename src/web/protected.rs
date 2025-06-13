use maud::{html, Markup, DOCTYPE};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::users::AuthSession;

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    use axum::response::Html;

    use super::*;

    pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => Html(protected_page(&user.username).into_string()).into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

fn protected_page(username: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { title { "Protected" } }
            body {
                p {
                    "Logged in as " (username) ", who has the "
                    code { "\"protected.read\"" }
                    " permission."
                }
            }
        }
    }
}
