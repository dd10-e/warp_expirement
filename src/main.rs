#![feature(use_extern_macros)]

extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate warp;

mod etablish_connection;
mod model;
mod repositery;
mod schema;

use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use model::user;
use repositery::user::{LoginForm, NewUser, RegisterForm};
use std::env;
use warp::{
    http,
    http::{header::HeaderValue, StatusCode},
    Filter,
};

/// Provides a RESTful web server managing a forum.
///
/// Actually the API is:
///
/// - `GET /users`: return a JSON list of all Users limited to 20.
/// - `GET /user/:id`: Access to a specific User.
/// - `POST /registration`: create a new User.
/// - `POST /login`: Authenticate as a User.

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "warp=trace");
    }
    pretty_env_logger::init();

    let users = warp::path("users");
    let user = warp::path("user");

    let users_index = users.and(warp::path::index());
    let user_id = user
        .and(warp::path::param::<i32>())
        .and(warp::path::index());

    // `GET /users`
    let list_all = warp::get(users_index.map(user::retrieve_users));

    // `GET /user/:id`
    let list_one = warp::get(user_id.and_then(|id| user::retrieve_one_user(id)));

    // `GET /registration`
    let register = warp::post(warp::path("registration").and(warp::body::form()).and_then(
        |form: RegisterForm| {
            if form.username.is_empty() || form.email.is_empty() || form.password.is_empty() || form.confirm_password.is_empty() {
                if form.confirm_password != form.password {
                    return Err(warp::reject::bad_request());
                }
                return Err(warp::reject::bad_request());
            }

            let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();

            let new_user = NewUser {
                username: &form.username,
                email: &form.email,
                hashed_password: &hashed_password,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };

            return user::add_user(new_user);
        },
    ));

    // `GET /login`
    let login = warp::path("login")
        .and(warp::body::form())
        .and_then(|form: LoginForm| {
            user::login(&form.username, &form.password)
                .and_then(|user_id| user::auth::generate_token(user_id))
                .map(|token| {
                    let jwt = format!("jwt={}", token.as_str());
                    http::Response::builder()
                        .header("Set-Cookie", &jwt[..])
                        .body(format!("{}", jwt))
                })
                .map_err(|_err| warp::reject::bad_request())
        });

    let check_user = warp::cookie("jwt").and_then(|auth: String| {
        let user_id = user::auth::verify_token(auth);
        user_id.map_err(|_err| warp::reject::bad_request())
    });

    // `GET /dashboard`
    let dashboard_path = warp::get(warp::path("dashboard"));
    let get_dashboard = dashboard_path.and(check_user).and_then(|user_id| {
        { http::Response::builder().body(format!("acces dashboard {}", user_id)) }
            .map_err(|_err| warp::reject::server_error())
    });

    let api = list_all
        .or(list_one)
        .or(register)
        .or(login)
        .or(get_dashboard);

    let routes = api.with(warp::log("api"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
