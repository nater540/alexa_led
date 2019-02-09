mod schema;
pub use schema::*;

use std::env;
use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn pool() -> Pool<ConnectionManager<PgConnection>> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
  let manager      = ConnectionManager::<PgConnection>::new(database_url);

  Pool::new(manager).expect("Error creating Postgres connection pool!")
}

pub struct Database(pub Pool<ConnectionManager<PgConnection>>);

unsafe impl Send for Database {}

impl Actor for Database {
  type Context = SyncContext<Self>;
}

// macro_rules! jsonb_column {
//   ($name: ident) => {
//     impl ::diesel::deserialize::FromSql<::diesel::sql_types::Jsonb, ::diesel::sql_types::Pg> for $name {
//       fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
//         let value = <::serde_json::Value as ::diesel::deserialize::FromSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg>>::from_sql(bytes)?;
//         Ok(::serde_json::from_value(value)?)
//       }
//     }

//     impl ::diesel::serialize::ToSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for $name {
//       fn to_sql<W: ::std::io::Write>(&self, out: &mut ::diesel::serialize::Output<W, Pg>) -> ::diesel::serialize::Result {
//         let value = ::serde_json::to_value(self)?;
//         <::serde_json::Value as ::diesel::serialize::ToSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg>>::to_sql(out)
//       }
//     }
//   }
// }
