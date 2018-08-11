use bcrypt::verify;
use diesel::{debug_query, pg::Pg, prelude::*};
use etablish_connection::*;
use repositery::user::{LoginForm, NewUser, PublicUserData, User};
use warp::{self, http::StatusCode};

pub fn retrieve_users() -> impl warp::Reply {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(20)
        .load::<User>(&connection)
        .expect("Error loading users");

    warp::reply::json(&results)
}

pub fn retrieve_one_user(id_param: i32) -> Result<impl warp::Reply, warp::Rejection> {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .select((id, email, username, created_at, updated_at))
        .find(id_param)
        .load::<PublicUserData>(&connection)
        .optional()
        .expect("Error loading user");

    match results {
        Some(json) => {
            if json.is_empty() == true {
                return Err(warp::reject::not_found());
            }
            Ok(warp::reply::json(&json))
        }
        None => Err(warp::reject::server_error()),
    }
}

pub fn add_user(user_form: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    use diesel::insert_into;
    use schema::users::dsl::*;

    let connection = establish_connection();
    let query = insert_into(users).values(&user_form);
    // println!("{}", debug_query::<Pg, _>(&query));
    query.execute(&connection).expect("Error insert user");

    Ok(StatusCode::CREATED)
}

pub fn login(username_form: &str, password_form: &str) -> Result<String, warp::Rejection> {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let result = users
        .select((id, email, username, hashed_password, created_at, updated_at))
        .filter(username.eq(&username_form))
        .first::<User>(&connection)
        .optional()
        .expect("Error loading user");

    match result {
        Some(result) => match verify(&password_form, &result.hashed_password) {
            Ok(valid) => {
                return {
                    let user_id = result.id.to_string();
                    Ok(user_id)
                    // let json = json!({
                    //     "code": "200",
                    //     "message": "Successful connection",
                    //     "user_id": &user_id
                    // });
                    // Ok(warp::reply::json(&json))
                };
            }
            Err(_) => return Err(warp::reject::bad_request()),
        },
        None => return Err(warp::reject::bad_request()),
    }
}

pub mod auth {
    use chrono::{DateTime, Utc};
    use jwt::{decode, encode, Header, Validation};
    use warp;

    // 30 days
    const EXPIRATION_TIME: i64 = 60 * 60 * 24 * 30;
    const SECRET_KEY: &str = "secret";

    #[derive(Serialize, Deserialize, Debug)]
    struct Claims {
        user_id: String,
        date: i64,
    }

    pub fn generate_token(user_id: String) -> Result<String, warp::Rejection> {
        let utc_now = Utc::now();

        let claims = Claims {
            user_id: user_id,
            date: utc_now.timestamp(),
        };

        match encode(&Header::default(), &claims, SECRET_KEY.as_ref()) {
            Ok(token) => return Ok(token),
            Err(_) => return Err(warp::reject::bad_request()),
        };
    }

    pub fn verify_token(token: String) -> Result<String, warp::Rejection> {
        let utc_now: DateTime<Utc> = Utc::now();

        match decode::<Claims>(&token, SECRET_KEY.as_ref(), &Validation::default()) {
            Ok(token) => {
                if token.claims.date + EXPIRATION_TIME < utc_now.timestamp() {
                    error!("Token is not longer valid !");
                    return Err(warp::reject::bad_request());
                }
                return Ok(token.claims.user_id);
            }
            Err(_) => {
                error!("Can't decode token !");
                return Err(warp::reject::bad_request());
            }
        };
    }
}
