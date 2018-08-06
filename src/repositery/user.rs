use chrono::NaiveDateTime;
use schema::users;

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PublicUserData {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub hashed_password: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
