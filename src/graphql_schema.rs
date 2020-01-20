extern crate diesel;
extern crate dotenv;

use crate::schema::members;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use juniper::FieldResult;
//use juniper::RootNode; //example

use juniper::{EmptyMutation, RootNode}; //tuto

// #[derive(juniper::GraphQLObject)]
// #[graphql(description="A member of the team")]
#[derive(Queryable)]
pub struct Member {
  pub id: i32,
  pub name: String,
	pub knockouts: i32,
	pub team_id: i32,
}

#[juniper::object(description = "A member of a team")] // new here
impl Member {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

	pub fn knockouts(&self) -> i32 {
		self.knockouts
	}

	pub fn team_id(&self) -> i32 {
		self.team_id
	}
}

#[derive(Queryable)]
  pub struct Team {
    pub id: i32,
    pub name: String,
  }
  
  #[juniper::object(description = "A team")]
  impl Team {
    pub fn id(&self) -> i32 {
      self.id
    }
  
    pub fn name(&self) -> &str {
      self.name.as_str()
    }
  
    pub fn members(&self) -> Vec<Member> {
      vec![]
    }
  }

pub struct QueryRoot;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
			.expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
			.expect(&format!("Error connecting to {}", database_url))
}

#[juniper::object]
impl QueryRoot  {
  fn members() -> Vec<Member> {
   use crate::schema::members::dsl::*;
   let connection = establish_connection();
   members
   	.limit(100)
   	.load::<Member>(&connection)
   	.expect("Error loading members")
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}
