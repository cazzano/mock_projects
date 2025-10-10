use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

// Example struct for demonstration
#[derive(sqlx::FromRow, serde::Serialize)]
struct User {
    id: i32,
    name: String,
}

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Server is running!")
}

// Example endpoint to test database connection
async fn get_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT id, name FROM users LIMIT 10")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch users")
        }
    }
}

// Example endpoint to insert a user
async fn create_user(
    pool: web::Data<Pool<Postgres>>,
    user_data: web::Json<serde_json::Value>,
) -> impl Responder {
    let name = user_data["name"].as_str().unwrap_or("Unknown");

    match sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind(name)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file (optional)
    dotenv::dotenv().ok();

    // Database connection string
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres@localhost/myapp".to_string()
    });

    println!("Connecting to database: {}", database_url);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("✓ Database connected successfully!");

    // Optional: Create a sample table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("✓ Database tables ready!");

    let server_address = "127.0.0.1:8080";
    println!("🚀 Starting server at http://{}", server_address);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(health_check))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind(server_address)?
    .run()
    .await
}
