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

http://diesel.rs/guides/getting-started/

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

INSTALL:

Pour Diesel (ORM) avec postgres,

==> docker run --name postgres -e POSTGRES_PASSWORD=password -p 127.0.0.1:5432:5432 -d postgres

sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://USER:PASSWORD@localhost/diesel_demo > .env
diesel setup
diesel migration generate NAME

Faire les modifs dans les fichier up.sql et down.sql: migrations/DATE_NAME/...

diesel migration run

Pour tester que notre down.sql est correct on peut: diesel migration redo



