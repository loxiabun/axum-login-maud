use maud::{html, Markup, DOCTYPE};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

use crate::users::{AuthSession, Credentials};

fn login_page(message: Option<String>, next: Option<String>) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Login" }
                style { r#"
                  label { display: block; margin-bottom: 5px; }
                "# }
            }
            body {
                @if let Some(message) = &message {
                    span { strong { (message) } }
                }
                form method="post" {
                    fieldset {
                        legend { "User login" }
                        p {
                            label for="username" { "Username" }
                            input name="username" id="username" value="ferris" {}
                        }
                        p {
                            label for="password" { "Password" }
                            input name="password" id="password" type="password" value="hunter42" {}
                        }
                    }
                    input type="submit" value="login" {}
                    @if let Some(next) = &next {
                        input type="hidden" name="next" value=(next) {}
                    }
                }
            }
        }
    }
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod post {
    use super::*;

    pub async fn login(
        mut auth_session: AuthSession,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Html(
                    login_page(
                        Some("Invalid credentials.".to_string()),
                        creds.next,
                    ).into_string(),
                )
                .into_response()
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if let Some(ref next) = creds.next {
            Redirect::to(next).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    }
}

mod get {
    use super::*;

    pub async fn login(Query(NextUrl { next }): Query<NextUrl>) -> Html<String> {
        Html(
            login_page(None, next).into_string(),
        )
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
