/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 * 
 * This file is part of the Auth Project.
 * 
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use std::io;

use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::cookie::Cookie;
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::web::Data;
use actix_web::web::Json;

use auth_service::Error;
use auth_service::dto::AccountDto;
use auth_service::dto::FormDto;
use auth_service::dto::UpdateDto;
use auth_service::service::AccountService;

use bcrypt::verify;

use validator::Validate;

#[get("/api/user")]
async fn get(service: Data<AccountService>, request: HttpRequest) -> Result<impl Responder, Error> {
    let read_key = match request.cookie("READ_KEY") {
        Some(read_key_cookie) => read_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No read key provided"));
        }
    };

    let mut transaction = service.transaction().await?;

    let entity = match transaction.find_by_read_key(&read_key).await? {
        Some(entity) => entity,
        None => {
            return Ok(HttpResponse::Forbidden().body("Invalid read key provided"));
        }
    };

    transaction.commit().await?;

    Ok(HttpResponse::Ok().json(AccountDto::from(entity)))
}

#[get("/api/user/authenticate")]
async fn authenticate(service: Data<AccountService>, request: HttpRequest) -> Result<impl Responder, Error> {
    let read_key = match request.cookie("READ_KEY") {
        Some(read_key_cookie) => read_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No read key provided"));
        }
    };

    let mut transaction = service.transaction().await?;

    if transaction.find_by_read_key(&read_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid read key provided"));
    }

    transaction.commit().await?;

    Ok(HttpResponse::Ok().body("Ok"))
}

#[post("/api/user/logout")]
async fn logout(service: Data<AccountService>, request: HttpRequest) -> Result<impl Responder, Error> {
    let write_key = match request.cookie("WRITE_KEY") {
        Some(write_key_cookie) => write_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No write key provided"));
        }
    };

    let mut transaction = service.transaction().await?;

    if transaction.find_by_write_key(&write_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid write key provided"));
    }

    let mut write_key_cookie = Cookie::build("WRITE_KEY", "").finish();
    let mut read_key_cookie = Cookie::build("READ_KEY", "").finish();

    write_key_cookie.make_removal();
    read_key_cookie.make_removal();

    transaction.commit().await?;

    Ok(
        HttpResponse::Ok()
        .cookie(write_key_cookie)
        .cookie(read_key_cookie)
        .body("Ok")
    )
}

#[post("/api/user/login")]
async fn login(service: Data<AccountService>, dto: Json<FormDto>) -> Result<impl Responder, Error> {
    dto.validate()?;
    
    let mut transaction = service.transaction().await?;
    
    let entity = match transaction.find_by_email(&dto.email).await? {
        Some(entity) => entity,
        None => {
            return Ok(HttpResponse::Forbidden().body("Invalid login data"));
        }
    };

    if !verify(&dto.password, &entity.password_hash)? {
        return Ok(HttpResponse::Forbidden().body("Invalid login data"));
    }

    let write_key_cookie = Cookie::build("WRITE_KEY", &entity.write_key).finish();
    let read_key_cookie = Cookie::build("READ_KEY", &entity.read_key).finish();

    transaction.commit().await?;

    Ok(
        HttpResponse::Ok()
        .cookie(write_key_cookie)
        .cookie(read_key_cookie)
        .body("Ok")
    )
}

#[post("/api/user/register")]
async fn register(service: Data<AccountService>, dto: Json<FormDto>) -> Result<impl Responder, Error> {
    dto.validate()?;

    let mut transaction = service.transaction().await?;

    transaction.lock().await?;

    let entity = match transaction.create(&dto.email, &dto.password).await {
        Ok(entity) => entity,
        Err(_) => {
            return Ok(HttpResponse::Conflict().body("Account already exists"));
        }
    };
    
    let write_key_cookie = Cookie::build("WRITE_KEY", &entity.write_key).finish();
    let read_key_cookie = Cookie::build("READ_KEY", &entity.read_key).finish();

    transaction.commit().await?;

    Ok(
        HttpResponse::Ok()
        .cookie(write_key_cookie)
        .cookie(read_key_cookie)
        .body("Ok")
    )
}

#[put("/api/user/update")]
async fn update(service: Data<AccountService>, request: HttpRequest, dto: Json<UpdateDto>) -> Result<impl Responder, Error> {
    dto.validate()?;

    let write_key = match request.cookie("WRITE_KEY") {
        Some(write_key_cookie) => write_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No write key provided"));
        }
    };

    let mut transaction = service.transaction().await?;

    if transaction.find_by_write_key(&write_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid write key provided"));
    }

    if transaction.update(&write_key, &dto.email, &dto.password).await.is_err() {
        return Ok(HttpResponse::Conflict().body("Email is already registered"));
    }

    transaction.commit().await?;

    Ok(HttpResponse::Ok().body("Ok"))
}

#[delete("/api/user/delete")]
async fn delete(service: Data<AccountService>, request: HttpRequest) -> Result<impl Responder, Error> {
    let write_key = match request.cookie("WRITE_KEY") {
        Some(write_key_cookie) => write_key_cookie.value().to_string(),
        None => {
            return Ok(HttpResponse::Forbidden().body("No write key provided"));
        }
    };

    let mut transaction = service.transaction().await?;

    if transaction.find_by_write_key(&write_key).await?.is_none() {
        return Ok(HttpResponse::Forbidden().body("Invalid write key provided"));
    }

    transaction.delete(&write_key).await?;
    transaction.commit().await?;

    Ok(HttpResponse::Ok().body("Ok"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let service = AccountService::new();

        App::new()
        .app_data(Data::new(service))
        .service(get)
        .service(authenticate)
        .service(logout)
        .service(login)
        .service(register)
        .service(update)
        .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
