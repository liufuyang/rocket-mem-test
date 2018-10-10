#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use rocket::fairing::AdHoc;
use rocket::State;
use rocket_contrib::Json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

struct UserState {
    map: Arc<Mutex<HashMap<String, User>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    token1: String,
    token2: String,
    token3: String,
    token4: String,
    token5: String,
    token6: String,
    token7: String,
    token8: String,
    token9: String,
    token10: String,
}

pub fn user_store_fairing() -> AdHoc {
    AdHoc::on_attach(|rocket| {
        let map: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));

        // Use Arc and it's clone method to safely send the cloned reference into another thread
        // while keeping the original reference available for creating managed State below.
        let thread_map = map.clone();

        thread::spawn(move || loop {
            thread::sleep(std::time::Duration::from_millis(1000));
            let mut _map = thread_map.lock().unwrap();
            for user in _map.values_mut() {
                user.token1 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token2 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token3 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token4 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token5 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token6 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token7 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token8 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token9 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
                user.token10 = Uuid::new_v4().to_string() + Uuid::new_v4().to_string().as_ref();
            }
        });

        let managed_rocket = rocket.manage(UserState { map: map });

        Ok(managed_rocket)
    })
}

#[get("/")]
fn index(state: State<UserState>) -> Json<HashMap<String, User>> {
    let map = state.map.lock().unwrap();

    Json(map.clone())
}

#[get("/login")]
fn login(state: State<UserState>) -> Json<HashMap<String, User>> {
    let mut map = state.map.lock().unwrap();

    map.insert(
        "1".to_owned(),
        User {
            token1: Uuid::new_v4().to_string(),
            token2: Uuid::new_v4().to_string(),
            token3: Uuid::new_v4().to_string(),
            token4: Uuid::new_v4().to_string(),
            token5: Uuid::new_v4().to_string(),
            token6: Uuid::new_v4().to_string(),
            token7: Uuid::new_v4().to_string(),
            token8: Uuid::new_v4().to_string(),
            token9: Uuid::new_v4().to_string(),
            token10: Uuid::new_v4().to_string(),
        },
    );

    Json(map.clone())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(user_store_fairing())
        .mount("/", routes![index, login])
}

fn main() {
    rocket().launch();
}
