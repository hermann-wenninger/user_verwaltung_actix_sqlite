use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use diesel::prelude::*;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
struct User {
    id: i32,
    name: String,
    email: String,
    role: String,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "profiles"]
struct Profile {
    user_id: i32,
    profile_photo: String,
    age: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

type LoggedInUsers = Arc<Mutex<HashSet<String>>>;

async fn login(user: web::Json<LoginRequest>, pool: web::Data<Pool<AsyncPgConnection>>, logged_in_users: web::Data<LoggedInUsers>) -> impl Responder {
    let stored_hash = "$2b$12$examplehashedpassword..."; // Beispielhash
    if verify(&user.password, stored_hash).unwrap() {
        let expiration = chrono::Utc::now().timestamp() as usize + 3600;
        let claims = Claims { sub: user.email.clone(), exp: expiration };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"))
            .unwrap();
        
        logged_in_users.lock().unwrap().insert(user.email.clone());
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

async fn register(user: web::Json<RegisterRequest>, pool: web::Data<Pool<AsyncPgConnection>>) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let new_user = User {
        id: 0, // Wird von der DB gesetzt
        name: user.name.clone(),
        email: user.email.clone(),
        role: user.role.clone(),
        password_hash: hashed_password,
    };
    HttpResponse::Created().json(new_user)
}

async fn list_users(pool: web::Data<Pool<AsyncPgConnection>>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = &mut pool.get().await.unwrap();
    let results = users.load::<User>(conn).await.unwrap();
    HttpResponse::Ok().json(results)
}

async fn list_logged_in_users(logged_in_users: web::Data<LoggedInUsers>) -> impl Responder {
    let users: Vec<String> = logged_in_users.lock().unwrap().iter().cloned().collect();
    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<AsyncPgConnection>::builder().build(database_url).await.unwrap();
    let logged_in_users: LoggedInUsers = Arc::new(Mutex::new(HashSet::new()));
    
    let server = HttpServer::new(move ||
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(logged_in_users.clone()))
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/users", web::get().to(list_users))
            .route("/logged-in-users", web::get().to(list_logged_in_users))
    );
    
    server.bind("127.0.0.1:8080")?.run().await
}
