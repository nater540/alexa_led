use crate::server::RagnarokState;

use actix_web::{Json, Result};

use serde_derive::{Deserialize, Serialize};

use log::info;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TestRequest {
  pub name: String
}

pub fn authenticate(
  (review, _state): (Json<TestRequest>, RagnarokState)
) -> Result<Json<TestRequest>> {
  info!("{:#?}", review);

  Ok(Json(TestRequest {
    ..Default::default()
  }))
}
