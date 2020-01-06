use juniper::FieldResult;
//use juniper::RootNode; //example

use juniper::{EmptyMutation, RootNode}; //tuto

#[derive(juniper::GraphQLObject)]
#[graphql(description="A member struct")]
struct Member {
  id: i32,
  name: String,
}

impl Member {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot  {
  fn members() -> Vec<Member> {
    vec![
      Member {
        id: 1,
        name: "Link".to_owned(),
      },
      Member {
        id: 2,
        name: "Mario".to_owned(),
      }
    ]
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}