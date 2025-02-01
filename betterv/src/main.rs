use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use actix_web::middleware::Logger;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    role: String,
    password_hash: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
    role: String,
}

const SECRET: &[u8] = b"supersecretkey";

lazy_static::lazy_static! {
    static ref LOGGED_IN_USERS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

async fn register_user(user_data: web::Json<RegisterRequest>) -> impl Responder {
    let conn = Connection::open("database.sqlite").expect("Datenbankverbindung fehlgeschlagen");
    let password_hash = user_data.password.clone(); // In Produktion bcrypt verwenden!
    let result = conn.execute(
        "INSERT INTO users (name, email, role, password_hash) VALUES (?1, ?2, ?3, ?4)",
        params![user_data.name, user_data.email, user_data.role, password_hash],
    );

    match result {
        Ok(_) => HttpResponse::Created().json("User successfully registered"),
        Err(_) => HttpResponse::BadRequest().body("User already exists or invalid data"),
    }
}

async fn list_users() -> impl Responder {
    let conn = Connection::open("database.sqlite").expect("Datenbankverbindung fehlgeschlagen");
    let mut stmt = conn.prepare("SELECT id, name, email, role FROM users").unwrap();
    let users_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            role: row.get(3)?,
            password_hash: "".to_string(),
        })
    }).unwrap();
    
    let users: Vec<User> = users_iter.filter_map(Result::ok).collect();
    HttpResponse::Ok().json(users)
}

async fn login(user_data: web::Json<LoginRequest>) -> impl Responder {
    let conn = Connection::open("database.sqlite").expect("Datenbankverbindung fehlgeschlagen");
    let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE email = ?1").unwrap();
    let user_iter = stmt.query_map(params![user_data.email], |row| {
        Ok(row.get::<_, String>(0)?)
    }).unwrap();
    
    if let Some(Ok(stored_hash)) = user_iter.into_iter().next() {
        if stored_hash == user_data.password { // In Produktion bcrypt verwenden!
            let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600;
            let claims = Claims { sub: user_data.email.clone(), exp: expiration as usize };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap();
            
            LOGGED_IN_USERS.lock().unwrap().push(user_data.email.clone());
            return HttpResponse::Ok().json(serde_json::json!({"token": token}));
        }
    }
    HttpResponse::Unauthorized().body("Invalid credentials")
}

async fn list_logged_in_users() -> impl Responder {
    let users = LOGGED_IN_USERS.lock().unwrap().clone();
    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/register", web::post().to(register_user))
            .route("/users", web::get().to(list_users))
            .route("/login", web::post().to(login))
            .route("/logged-in-users", web::get().to(list_logged_in_users))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
