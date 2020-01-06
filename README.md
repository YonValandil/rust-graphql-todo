ROUTES:
just a test for the http request: http://127.0.0.1:8080/{id}/{name}/
for graphql: http://127.0.0.1:8080/graphiql

SOURCES:

about Rust:
https://www.viget.com/articles/understanding-futures-in-rust-part-1/

tuto:
https://www.freecodecamp.org/news/building-powerful-graphql-servers-with-rust/
https://dev.to/open-graphql/building-powerful-graphql-servers-with-rust-3gla

example from doc actix:
https://github.com/actix/examples/blob/master/juniper/src/schema.rs
https://github.com/actix/examples/blob/master/juniper/src/main.rs

graphql doc:
https://graphql-rust.github.io/juniper/master/quickstart.html
https://graphql-rust.github.io/juniper/master/index.html


DEPENDENCIES:

[dependencies]
actix-web ="2.0"
actix-rt = "1.0.0"
diesel = { version = "1.0.0", features = ["postgres"]}
dotenv = "0.9.0"
env_logger ="0.7.1"
futures = "0.3.1"
juniper = "0.14.2"
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"

for the freecodecamp tuto:
actix-web = "1.0.0"
