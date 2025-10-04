use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MoneyResponse {
    money: String,
    status: u16,
}

async fn add_money(req: HttpRequest) -> impl Responder {
    let money = req
        .headers()
        .get("money")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("5645455")
        .to_string();

    HttpResponse::Ok().json(MoneyResponse { money, status: 200 })
}

pub fn add_money_blueprint(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/add").route(web::get().to(add_money)));
}
