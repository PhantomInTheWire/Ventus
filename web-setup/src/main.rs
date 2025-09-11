use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize)]
struct ShellResponse {
    success: bool,
    output: String,
    error: String,
}

#[derive(Deserialize)]
struct SetupParams {
    file_path: String,
    host: String,
}

#[get("/run-setup")]
async fn run_setup(params: web::Query<SetupParams>) -> impl Responder {
    let output = Command::new("./artifact")
        .args([
            "sync",
            "--host", &params.host,
            "--port", "1234",
            "--local-dir", &params.file_path,
            "--remote-dir", "files/"
        ])
        .output();

    match output {
        Ok(output) => {
            let success = output.status.success();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            if !success {
                return HttpResponse::BadRequest().json(ShellResponse {
                    success: false,
                    output: stdout,
                    error: stderr,
                });
            }

            HttpResponse::Ok().json(ShellResponse {
                success: true,
                output: stdout,
                error: stderr,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ShellResponse {
                success: false,
                output: String::new(),
                error: e.to_string(),
            })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(run_setup)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
