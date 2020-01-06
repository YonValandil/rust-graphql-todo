Sources:

tuto:
https://www.freecodecamp.org/news/building-powerful-graphql-servers-with-rust/

example from doc:
https://github.com/actix/examples/blob/master/juniper/src/schema.rs
https://github.com/actix/examples/blob/master/juniper/src/main.rs

https://graphql-rust.github.io/juniper/master/quickstart.html

dependencies:

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
