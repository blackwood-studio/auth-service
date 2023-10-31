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
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::cookie::Cookie;
use actix_web::get;
use actix_web::post;
use actix_web::web::Data;
use actix_web::web::Json;

use auth_service::dto::AccountDto;
use auth_service::dto::AuthenticateDto;
use auth_service::service::AccountService;

use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;

use validator::Validate;

#[get("/api/user")]
async fn get(pool: Data<PgPool>, request: HttpRequest) -> auth_service::Result<impl Responder> {
    let read_key = match request.cookie("READ_KEY") {
        Some(read_key_cookie) => read_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No read key provided"));
        },
    };

    let entity = match AccountService::find_by_read_key(&pool, &read_key).await? {
        Some(entity) => entity,
        None => {
            return Ok(HttpResponse::Forbidden().body("Invalid read key provided"));
        }
    };

    Ok(HttpResponse::Ok().json(AccountDto::from(entity)))
}

#[get("/api/user/authenticate")]
async fn authenticate(pool: Data<PgPool>, request: HttpRequest) -> auth_service::Result<impl Responder> {
    let read_key = match request.cookie("READ_KEY") {
        Some(read_key_cookie) => read_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No read key provided"));
        }
    };

    if AccountService::find_by_read_key(&pool, &read_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid read key provided"));
    }

    Ok(HttpResponse::Ok().body("Ok"))
}

#[get("/api/user/delete")]
async fn delete(pool: Data<PgPool>, request: HttpRequest) -> auth_service::Result<impl Responder> {
    let write_key = match request.cookie("WRITE_KEY") {
        Some(write_key_cookie) => write_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No write key provided"));
        },
    };

    if AccountService::find_by_write_key(&pool, &write_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid write key provided"));
    }

    AccountService::delete(&pool, &write_key).await?;

    Ok(HttpResponse::Ok().body("Ok"))
}

#[post("/api/user/login")]
async fn login(pool: Data<PgPool>, dto: Json<AuthenticateDto>) -> auth_service::Result<impl Responder> {
    dto.validate()?;
    
    let entity = match AccountService::find_by_email(&pool, &dto.email).await? {
        Some(entity) => entity,
        None => {
            return Ok(HttpResponse::Forbidden().body("Invalid login data"));
        }
    };

    if !entity.verify(&dto.password)? {
        return Ok(HttpResponse::Forbidden().body("Invalid login data"));
    }

    let write_key_cookie = Cookie::build("WRITE_KEY", &entity.write_key).finish();
    let read_key_cookie = Cookie::build("READ_KEY", &entity.read_key).finish();

    Ok(
        HttpResponse::Ok()
        .cookie(write_key_cookie)
        .cookie(read_key_cookie)
        .body("Ok")
    )
}

#[post("/api/user/register")]
async fn register(pool: Data<PgPool>, dto: Json<AuthenticateDto>) -> auth_service::Result<impl Responder> {
    dto.validate()?;

    let entity = match AccountService::create(&pool, &dto.email, &dto.password).await {
        Ok(entity) => entity,
        Err(_) => {
            return Ok(HttpResponse::Conflict().body("Account already exists"));
        },
    };
    
    let write_key_cookie = Cookie::build("WRITE_KEY", &entity.write_key).finish();
    let read_key_cookie = Cookie::build("READ_KEY", &entity.read_key).finish();

    Ok(
        HttpResponse::Ok()
        .cookie(write_key_cookie)
        .cookie(read_key_cookie)
        .body("Ok")
    )
}

#[post("/api/user/update")]
async fn update(pool: Data<PgPool>, request: HttpRequest, dto: Json<AuthenticateDto>) -> auth_service::Result<impl Responder> {
    dto.validate()?;

    let write_key = match request.cookie("WRITE_KEY") {
        Some(write_key_cookie) => write_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No write key provided"));
        },
    };

    if AccountService::find_by_write_key(&pool, &write_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid write key provided"));
    }

    if AccountService::update(&pool, &write_key, &dto.email, &dto.password).await.is_err() {
        return Ok(HttpResponse::Conflict().body("Email is already registered"));
    }

    Ok(HttpResponse::Ok().body("Ok"))
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
