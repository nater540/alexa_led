use std::sync::Arc;

use actix_web::{http, server::HttpServer, App, State};
use actix::prelude::*;

use crate::errors::*;
use crate::Settings;

mod routes;
use routes::authenticate;

pub struct AppState {
  pub db: Addr<crate::db::Database>,
  //pub settings: Settings
}

pub type RagnarokState = State<Arc<AppState>>;

pub fn start(settings: &Settings) -> Result<()> {
  let pool = crate::db::pool();
  let addr = SyncArbiter::start(12, move || crate::db::Database(pool.clone()));

  let state = Arc::new(AppState {
    db: addr.clone(),
    //settings: settings.clone()
  });

  let ragnarok_app = move || {
    App::with_state(state.clone())
      .prefix("/api")
      .resource("/authenticate", |res| {
        res.method(http::Method::POST).with(authenticate)
      })
  };

  HttpServer::new(ragnarok_app)
    .backlog(9000)
    .workers(4)
    .bind(settings.inbound_listener.address)
    .unwrap()
    .start();

  Ok(())
}
