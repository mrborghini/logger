mod components;

use std::{env, num::ParseIntError};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use components::{types::Severity, DotEnvReader, Logger};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct JsonResponse {
    pub message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(JsonResponse { message: "Hello, World!".to_string() })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_reader = DotEnvReader::new(".env");
    env_reader.parse_and_set_env();

    let logger = Logger::new("main");

    let address_env = env::var("BACKEND_HOST_URL");
    let address: String;

    match address_env {
        Ok(out) => {
            address = out;
        }
        Err(e) => {
            logger.error(
                format!("Expected BACKEND_HOST_URL in enviroment: {}", e).as_str(),
                "main",
                Severity::Critical,
            );
            std::process::exit(1);
        }
    }

    let port_env = env::var("BACKEND_HOST_PORT");

    let port: u16;

    match port_env {
        Ok(port_string) => {
            let parsed_port: Result<u16, ParseIntError> = port_string.parse();

            match parsed_port {
                Ok(out) => {
                    port = out;
                }
                Err(e) => {
                    logger.error(
                        format!(
                            "Expected BACKEND_HOST_PORT in environment with an int format: {}",
                            e
                        )
                        .as_str(),
                        "main",
                        Severity::Critical,
                    );
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            logger.error(
                format!("Expected BACKEND_HOST_PORT in enviroment: {}", e).as_str(),
                "main",
                Severity::Critical,
            );
            std::process::exit(1);
        }
    }

    logger.info(
        format!("Starting up API: {}:{}", address, port).as_str(),
        "main",
    );

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((address, port))?
    .run()
    .await
}
