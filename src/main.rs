#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use rocket_contrib::Json;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    token: String,
}

#[get("/")]
fn index() -> String {
    "Good".to_owned()
}

#[get("/login")]
fn login() -> Json<HashMap<String, User>> {
    let mut map = HashMap::new();

    let uuid = Uuid::new_v4();

    map.insert(
        "1".to_owned(),
        User {
            token: uuid.to_string(),
        },
    );

    Json(map.clone())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, login])
}

fn main() {
    rocket().launch();
}
