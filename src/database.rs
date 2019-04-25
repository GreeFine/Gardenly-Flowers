use super::graphql::types::{RequestableObjects, ConnectionTrait};
use juniper::GraphQLType;
use mysql::OptsBuilder;

pub struct Database {
  pool: mysql::Pool,
}


#[derive(AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ETables {
  pictures,
  descriptions,
}

impl Database {

  pub fn new() -> Database {
    let mut database = OptsBuilder::new();
    database
      .user(Some("greefine"))
      .pass(Some("password"))
      .db_name(Some("Flowers"));
    Database {
      pool: mysql::Pool::new(database).unwrap(),
    }
  }

  pub fn request<T>(&self, table: ETables, limit: Option<i32>) -> T
  where
    T: ConnectionTrait,
    T: GraphQLType,
    T: Default,
    T: RequestableObjects,
  {
    let mut request = String::new();
    let mut data = T::default();

    let fields = data.field_names();
    for field in fields {
      request.push_str(field);
      request.push_str(", ");
    }
    request.truncate(request.len() - 2);
    match limit {
      None => request = format!("SELECT {0} FROM {1};", request, table.as_ref()),
      Some(x) => request = format!("SELECT {0} FROM {1} LIMIT {2};", request, table.as_ref(), x),
    }

    let rows = self
      .pool
      .prep_exec(request, ())
      .map(|result| result.map(|x| x.unwrap()))
      .unwrap();
    for mut row in rows {
      ConnectionTrait::feed(&mut data, &mut row);
    }

    data
  }
}

