use std::env;
use std::sync::Mutex;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result, get, post, http::header, error::ErrorInternalServerError};
use mysql::*;
use mysql::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

// Define structs for URL mappings
#[derive(Debug)]
struct UrlMapping {
    short_url: String,
    long_url: String,
}

// For request serialization
#[derive(Deserialize)]
struct CreateShortenRequest {
    url: String,
}

// For response serialization
#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
    original_url: String,
}

// Wrap our pool in an app state for actix
struct AppState {
    pool: Mutex<Pool>,
}

#[post("/shorten")]
async fn create_short_url(
    data: web::Data<AppState>,
    req: web::Json<CreateShortenRequest>,
) -> Result<impl Responder> {
    // Validate the URL
    if !req.url.starts_with("http") {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "URL must start with http:// or https://"
        })));
    }

    // Generate a random short URL
    let mut rng = rand::thread_rng();
    let short_url: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) as char)
        .take(8)
        .collect();

    // Store in database
    let pool = data.pool.lock().unwrap();
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database connection failed"
            })));
        }
    };

    match conn.exec_drop(
        "INSERT INTO url_mappings (short_url, long_url) VALUES (?, ?)",
        (&short_url, &req.url)
    ) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to store URL mapping"
            })));
        }
    }

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let short_url_full = format!("{}/{}", base_url, short_url);

    Ok(HttpResponse::Created().json(ShortenResponse {
        short_url: short_url_full,
        original_url: req.url.clone(),
    }))
}

#[get("/{short_url}")]
async fn redirect(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let short_url = path.into_inner();
    
    // Look up the URL in the database
    let pool = data.pool.lock().unwrap();
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database connection failed"
            })));
        }
    };

    let result: Vec<UrlMapping> = match conn.exec_map(
        "SELECT short_url, long_url FROM url_mappings WHERE short_url = ?",
        (short_url,),
        |(short_url, long_url)| UrlMapping { short_url, long_url }
    ) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Database query error: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to query URL mapping"
            })));
        }
    };

    if let Some(mapping) = result.first() {
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, mapping.long_url.clone()))
            .finish())
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Short URL not found"
        })))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get database connection parameters
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://shortener_user:s3cur3_p4ssw0rd@127.0.0.1:3306/url_shortener".to_string());
    
    // Create database pool
    let opts = Opts::from_url(&db_url).expect("Invalid database URL");
    let pool = Pool::new(opts).expect("Failed to create database pool");
    
    // Initialize the database table
    let mut conn = pool.get_conn().expect("Failed to get database connection");
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS url_mappings (
            id INT AUTO_INCREMENT PRIMARY KEY,
            short_url VARCHAR(8) UNIQUE NOT NULL,
            long_url TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).expect("Failed to create database table");

    // Get server host and port
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>().expect("PORT must be a number");
    
    println!("Starting server at http://{}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                pool: Mutex::new(pool.clone()),
            }))
            .service(create_short_url)
            .service(redirect)
    })
    .bind((host, port))?
    .run()
    .await
}