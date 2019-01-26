use std::sync::Arc;

use actix_web::{http, server::HttpServer, App, State};

use super::errors::*;

mod routes;
use routes::authenticate;

#[derive(Debug)]
pub struct AppState {
  pub settings: String
}

pub type RagnarokState = State<Arc<AppState>>;

pub fn start(address: &str) -> Result<()> {
  let state = Arc::new(AppState {
    // settings: Settings::new("config/*.{yml|yaml}").unwrap()
    settings: "Kitty!".to_string()
  });

  let ragnarok_app = move || {
    App::with_state(state.clone())
      .prefix("/api")
      .resource("/authenticate", |res| {
        res.method(http::Method::POST).with(authenticate)
      })
  };

  HttpServer::new(ragnarok_app).bind(address).unwrap().start();

  Ok(())
}
