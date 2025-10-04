use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;
use std::sync::Mutex;

mod apis;
use apis::dummy;

#[derive(Serialize)]
struct Response {
    message: String,
    status: u16,
}

#[derive(Serialize)]
struct WelcomeResponse {
    message: String,
    endpoints: Vec<String>,
}

// Fuck You Blueprint
async fn fuck_you() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "Fuck You !!!".to_string(),
        status: 200,
    })
}

// Health Blueprint
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "fuck-you-api"
    }))
}

// Root endpoint
async fn index() -> impl Responder {
    HttpResponse::Ok().json(WelcomeResponse {
        message: "Welcome to Fuck You API".to_string(),
        endpoints: vec![
            "GET /api/fuck_you".to_string(),
            "GET /api/health".to_string(),
            "POST /api/add".to_string(),
            "GET /api/gangsters".to_string(),
            "GET /api/gangsters/{id}".to_string(),
            "POST /api/add".to_string(),
            "PUT /api/gangsters/{id}".to_string(),
            "DELETE /api/gangsters/{id}".to_string(),
        ],
    })
}

// Fuck You Blueprint Configuration
pub fn fuck_you_blueprint(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/fuck_you").route(web::get().to(fuck_you)));
}

// Health Blueprint Configuration
pub fn health_blueprint(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health)));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    println!("\nMy Fucking App\n");
    
    // Initialize database
    let db = match dummy::init_db() {
        Ok(database) => {
            log::info!("✅ Database initialized successfully");
            database
        }
        Err(e) => {
            log::error!("Failed to initialize database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    
    let db_data = web::Data::new(Mutex::new(db));
    
    log::info!("Server starting on :3000");
    
    HttpServer::new(move || {
        let cors = Cors::permissive();
        
        App::new()
            .app_data(db_data.clone())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                    .configure(fuck_you_blueprint)
                    .configure(health_blueprint)
                    .configure(dummy::add_money_blueprint)
                    .configure(dummy::gangster_blueprint)
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
