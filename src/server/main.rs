use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use markdown;

async fn list_tasks(info: web::Path<(String, String)>) -> impl Responder {
    let (topic, labels) = info.into_inner();
    let output = Command::new("st")
        .args(["list", "-T", &topic, "-Mnvl", &labels])
        .output()
        .expect("Failed to execute st command");
    let response_body = String::from_utf8_lossy(&output.stdout).to_string();
    HttpResponse::Ok().body(markdown::to_html(&response_body))
}

async fn list_all_tasks(info: web::Path<(String,)>) -> impl Responder {
    let (topic,) = info.into_inner();
    let output = Command::new("st")
        .args(["list", "-T", &topic, "-Mnv"])
        .output()
        .expect("Failed to execute st command");
    let response_body = String::from_utf8_lossy(&output.stdout).to_string();
    HttpResponse::Ok().body(markdown::to_html(&response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/list/{topic}/{labels}", web::get().to(list_tasks))
            .route("/list/{topic}", web::get().to(list_all_tasks))
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
