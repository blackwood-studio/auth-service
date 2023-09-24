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

use std::io;

use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::cookie::Cookie;
use actix_web::get;
use actix_web::post;
use actix_web::web::Data;
use actix_web::web::Json;

use auth_service::form::Register;
use auth_service::service::Accounts;

use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;
use validator::Validate;

#[get("/api/user")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("/api/user")
}

#[get("/api/user/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("/api/user/login")
}

#[post("/api/user/register")]
async fn register(pool: Data<PgPool>, register: Json<Register>) -> auth_service::Result<impl Responder> {
    register.validate()?;
    
    if Accounts::find_by_email(&pool, &register.email).await?.is_some() {
        return Ok(HttpResponse::Conflict().body("Account already exists"));
    }

    let account = Accounts::create(pool, &register.email, &register.password).await?;
    let write_key_cookie = Cookie::build("WRITE_KEY", &account.write_key).finish();
    let read_key_cookie = Cookie::build("READ_KEY", &account.read_key).finish();

    Ok(HttpResponse::Ok().cookie(write_key_cookie).cookie(read_key_cookie).body("Ok"))
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
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let pg_options = PgConnectOptions::new()
        .host(dotenv!("HOST_ADDRESS"))
        .database(dotenv!("DATABASE_NAME"))
        .username(dotenv!("DATABASE_USER_NAME"))
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
