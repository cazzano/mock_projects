use actix_web::{web, HttpRequest, HttpResponse, Responder};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct Gangster {
    gangster_id: String,
    gangster_name: String,
}

#[derive(Serialize)]
struct Response<T> {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("gangsters.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS gangsters (
            gangster_id TEXT PRIMARY KEY,
            gangster_name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    Ok(conn)
}

fn generate_gangster_id(conn: &Connection) -> Result<String> {
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM gangsters", [], |row| row.get(0))?;
    Ok(format!("G{:02}", count + 1))
}

// POST - Add new gangster
async fn add_gangster(
    req: HttpRequest,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let gangster_name = match req.headers().get("gangster") {
        Some(val) => match val.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => {
                return HttpResponse::BadRequest().json(Response::<()> {
                    success: false,
                    message: "Invalid gangster header".to_string(),
                    data: None,
                })
            }
        },
        None => {
            return HttpResponse::BadRequest().json(Response::<()> {
                success: false,
                message: "gangster header is required".to_string(),
                data: None,
            })
        }
    };

    let conn = db.lock().unwrap();

    let gangster_id = match generate_gangster_id(&conn) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::<()> {
                success: false,
                message: "Failed to generate gangster ID".to_string(),
                data: None,
            })
        }
    };

    match conn.execute(
        "INSERT INTO gangsters (gangster_id, gangster_name) VALUES (?1, ?2)",
        [&gangster_id, &gangster_name],
    ) {
        Ok(_) => HttpResponse::Created().json(Response {
            success: true,
            message: "Gangster added successfully".to_string(),
            data: Some(Gangster {
                gangster_id,
                gangster_name,
            }),
        }),
        Err(_) => HttpResponse::Conflict().json(Response::<()> {
            success: false,
            message: "Gangster already exists or database error".to_string(),
            data: None,
        }),
    }
}

// GET - Get all gangsters
async fn get_gangsters(db: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = db.lock().unwrap();

    let mut stmt = match conn.prepare("SELECT gangster_id, gangster_name FROM gangsters") {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::<()> {
                success: false,
                message: "Failed to fetch gangsters".to_string(),
                data: None,
            })
        }
    };

    let gangsters_iter = stmt.query_map([], |row| {
        Ok(Gangster {
            gangster_id: row.get(0)?,
            gangster_name: row.get(1)?,
        })
    });

    let gangsters: Vec<Gangster> = match gangsters_iter {
        Ok(iter) => iter.filter_map(Result::ok).collect(),
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::<()> {
                success: false,
                message: "Failed to fetch gangsters".to_string(),
                data: None,
            })
        }
    };

    HttpResponse::Ok().json(Response {
        success: true,
        message: "Gangsters fetched successfully".to_string(),
        data: Some(gangsters),
    })
}

// GET - Get single gangster by ID
async fn get_gangster(
    path: web::Path<String>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let id = path.into_inner();
    let conn = db.lock().unwrap();

    match conn.query_row(
        "SELECT gangster_id, gangster_name FROM gangsters WHERE gangster_id = ?1",
        [&id],
        |row| {
            Ok(Gangster {
                gangster_id: row.get(0)?,
                gangster_name: row.get(1)?,
            })
        },
    ) {
        Ok(gangster) => HttpResponse::Ok().json(Response {
            success: true,
            message: "Gangster fetched successfully".to_string(),
            data: Some(gangster),
        }),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            HttpResponse::NotFound().json(Response::<()> {
                success: false,
                message: "Gangster not found".to_string(),
                data: None,
            })
        }
        Err(_) => HttpResponse::InternalServerError().json(Response::<()> {
            success: false,
            message: "Database error".to_string(),
            data: None,
        }),
    }
}

// PUT - Update gangster
async fn update_gangster(
    req: HttpRequest,
    path: web::Path<String>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let id = path.into_inner();

    let new_name = match req.headers().get("gangster") {
        Some(val) => match val.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => {
                return HttpResponse::BadRequest().json(Response::<()> {
                    success: false,
                    message: "Invalid gangster header".to_string(),
                    data: None,
                })
            }
        },
        None => {
            return HttpResponse::BadRequest().json(Response::<()> {
                success: false,
                message: "gangster header is required".to_string(),
                data: None,
            })
        }
    };

    let conn = db.lock().unwrap();

    match conn.execute(
        "UPDATE gangsters SET gangster_name = ?1 WHERE gangster_id = ?2",
        [&new_name, &id],
    ) {
        Ok(rows) => {
            if rows == 0 {
                HttpResponse::NotFound().json(Response::<()> {
                    success: false,
                    message: "Gangster not found".to_string(),
                    data: None,
                })
            } else {
                HttpResponse::Ok().json(Response {
                    success: true,
                    message: "Gangster updated successfully".to_string(),
                    data: Some(Gangster {
                        gangster_id: id,
                        gangster_name: new_name,
                    }),
                })
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Response::<()> {
            success: false,
            message: "Failed to update gangster".to_string(),
            data: None,
        }),
    }
}

// DELETE - Delete gangster
async fn delete_gangster(
    path: web::Path<String>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let id = path.into_inner();
    let conn = db.lock().unwrap();

    match conn.execute("DELETE FROM gangsters WHERE gangster_id = ?1", [&id]) {
        Ok(rows) => {
            if rows == 0 {
                HttpResponse::NotFound().json(Response::<()> {
                    success: false,
                    message: "Gangster not found".to_string(),
                    data: None,
                })
            } else {
                HttpResponse::Ok().json(Response::<()> {
                    success: true,
                    message: "Gangster deleted successfully".to_string(),
                    data: None,
                })
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Response::<()> {
            success: false,
            message: "Failed to delete gangster".to_string(),
            data: None,
        }),
    }
}

pub fn gangster_blueprint(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/add", web::post().to(add_gangster))
            .route("/gangsters", web::get().to(get_gangsters))
            .route("/gangsters/{id}", web::get().to(get_gangster))
            .route("/gangsters/{id}", web::put().to(update_gangster))
            .route("/gangsters/{id}", web::delete().to(delete_gangster)),
    );
}
