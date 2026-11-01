//! Server-side sessions with tower-sessions: a counter, login with a new session id,
//! flash messages, and CSRF protection with `CsrfLayer` + `SameSite`.

use axum::{
    Form, Router,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::csrf::CsrfLayer;
use tower_sessions::{
    Expiry, MemoryStore, Session, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};

// --- the simplest: a counter ----------------------------------------------------

async fn visits(session: Session) -> Result<String, StatusCode> {
    let n: u32 = session.get("visits").await.map_err(internal)?.unwrap_or(0);
    session.insert("visits", n + 1).await.map_err(internal)?;
    Ok(format!("visit number {}", n + 1))
}

fn internal(e: tower_sessions::session::Error) -> StatusCode {
    tracing::error!(error = %e, "session store");
    StatusCode::INTERNAL_SERVER_ERROR
}

// --- flash: `->with('status', ...)` from Laravel -------------------------------

const FLASH: &str = "_flash";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flash {
    pub level: String,
    pub text: String,
}

/// Put a message for the NEXT request.
pub async fn flash(session: &Session, level: &str, text: &str) {
    let msg = Flash {
        level: level.into(),
        text: text.into(),
    };
    let _ = session.insert(FLASH, msg).await;
}

/// Read it once: `remove` returns the value and deletes it in one step.
pub async fn take_flash(session: &Session) -> Option<Flash> {
    session.remove::<Flash>(FLASH).await.ok().flatten()
}

// --- login: a new session id against session fixation ---------------------------

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn login(session: Session, Form(f): Form<LoginForm>) -> impl IntoResponse {
    if f.password != "secret" {
        flash(&session, "error", "wrong email or password").await;
        return Redirect::to("/login");
    }
    // the id an attacker could have planted before login stops working
    if session.cycle_id().await.is_err() {
        return Redirect::to("/login");
    }
    let _ = session.insert("user", &f.email).await;
    flash(&session, "success", "welcome back").await;
    Redirect::to("/dashboard")
}

async fn dashboard(session: Session) -> Result<String, Redirect> {
    let user: Option<String> = session.get("user").await.ok().flatten();
    let Some(user) = user else {
        return Err(Redirect::to("/login"));
    };
    let note = take_flash(&session)
        .await
        .map(|f| f.text)
        .unwrap_or_default();
    Ok(format!("dashboard of {user} {note}"))
}

async fn login_page(session: Session) -> String {
    match take_flash(&session).await {
        Some(f) => format!("login form [{}: {}]", f.level, f.text),
        None => "login form".into(),
    }
}

async fn logout(session: Session) -> Redirect {
    let _ = session.flush().await; // delete the data AND the cookie
    Redirect::to("/login")
}

pub fn app() -> Router {
    let store = MemoryStore::default(); // lost on restart; see the chapter for real stores
    let sessions = SessionManagerLayer::new(store)
        .with_name("app_session")
        .with_secure(false) // dev over http://; leave the default `true` in production
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(2)));

    Router::new()
        .route("/visits", get(visits))
        .route("/login", get(login_page).post(login))
        .route("/dashboard", get(dashboard))
        .route("/logout", post(logout))
        .layer(CsrfLayer::new())
        .layer(sessions)
}
