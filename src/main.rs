/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 * 
 * This file is part of the Auth Project.
 * 
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

#[macro_use]
extern crate dotenv_codegen;

use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::web::Data;

use auth_service::Result;

use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;

#[get("/api/user")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("/api/user")
}

#[get("/api/user/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("/api/user/login")
}

#[get("/api/user/register")]
async fn register() -> impl Responder {
    HttpResponse::Ok().body("/api/user/register")
}

#[get("/api/user/authenticate")]
async fn authenticate() -> impl Responder {
    HttpResponse::Ok().body("/api/user/authenticate")
}

#[get("/api/user/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("/api/user/update")
}

#[get("/api/user/delete")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("/api/user/delete")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let pg_options = PgConnectOptions::new()
        .host("localhost")
        .database("user")
        .username(dotenv!("DATABASE_USER"))
        .password(dotenv!("DATABASE_USER_PASSWORD"));

        let pool = PgPool::connect_lazy_with(pg_options);

        App::new()
        .app_data(Data::new(pool))
        .service(get)
        .service(login)
        .service(register)
        .service(authenticate)
        .service(update)
        .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
